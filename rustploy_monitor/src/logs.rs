use tracing::error;

use crate::docker::api::DockerApi;

/// Header length of a frame in docker's multiplexed stream format.
const FRAME_HEADER_LEN: usize = 8;
/// Byte 0 of the header: 1 = stdout, 2 = stderr.
const STREAM_STDERR: u8 = 2;

/// Reads the last `tail_lines` lines of a container's logs via the Docker API.
///
/// stdout and stderr are merged because docker interleaves them and callers only
/// need the combined view.
pub async fn tail_container_logs(
    api: &DockerApi,
    container_id: &str,
    tail_lines: usize,
) -> Vec<String> {
    let path = format!(
        "/containers/{container_id}/logs?stdout=true&stderr=true&tail={tail_lines}"
    );

    let body = match api.get_raw(&path).await {
        Ok(body) => body,
        Err(error) => {
            error!(%error, container_id, "could not read container logs");
            return Vec::new();
        }
    };

    demux_log_stream(&body)
        .into_iter()
        .map(|(_, line)| line)
        .collect()
}

/// Splits docker's log stream into `(is_stderr, line)` pairs.
///
/// Containers without a TTY get a multiplexed stream: each chunk is preceded by
/// an 8-byte header of `[stream_type, 0, 0, 0, len_be_u32]`. TTY containers get
/// raw text with no framing, so the format is detected rather than assumed —
/// treating raw text as framed would slice binary garbage out of real log lines.
fn demux_log_stream(body: &[u8]) -> Vec<(bool, String)> {
    if !looks_multiplexed(body) {
        return body
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| (false, String::from_utf8_lossy(line).trim_end().to_string()))
            .collect();
    }

    let mut lines = Vec::new();
    let mut offset = 0usize;

    while offset + FRAME_HEADER_LEN <= body.len() {
        let stream_type = body[offset];
        let length = u32::from_be_bytes([
            body[offset + 4],
            body[offset + 5],
            body[offset + 6],
            body[offset + 7],
        ]) as usize;

        let start = offset + FRAME_HEADER_LEN;
        // A truncated final frame is normal when a tail cuts mid-frame.
        let end = start.saturating_add(length).min(body.len());
        if start >= end {
            break;
        }

        let is_stderr = stream_type == STREAM_STDERR;
        for line in body[start..end].split(|b| *b == b'\n') {
            if line.is_empty() {
                continue;
            }
            lines.push((
                is_stderr,
                String::from_utf8_lossy(line).trim_end().to_string(),
            ));
        }

        offset = end;
    }

    lines
}

/// Heuristic for docker's frame header: byte 0 is a known stream type and
/// bytes 1-3 are zero padding.
///
/// The declared length is deliberately *not* checked against the buffer: a tail
/// can cut the stream mid-frame, leaving a header that promises more bytes than
/// arrived. Rejecting those would misread the frame header as log text and emit
/// control bytes into the output.
///
/// Plain log text failing this is overwhelmingly likely — byte 0 would have to
/// be 0x00-0x02, a control character no text log starts with.
fn looks_multiplexed(body: &[u8]) -> bool {
    if body.len() < FRAME_HEADER_LEN {
        return false;
    }

    let stream_type = body[0];
    if stream_type > STREAM_STDERR {
        return false;
    }

    body[1] == 0 && body[2] == 0 && body[3] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds one multiplexed frame.
    fn frame(stream_type: u8, payload: &str) -> Vec<u8> {
        let mut out = vec![stream_type, 0, 0, 0];
        out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        out.extend_from_slice(payload.as_bytes());
        out
    }

    #[test]
    fn demuxes_stdout_and_stderr_frames() {
        let mut body = frame(1, "hello from stdout\n");
        body.extend(frame(2, "oops from stderr\n"));

        let lines = demux_log_stream(&body);
        assert_eq!(
            lines,
            vec![
                (false, "hello from stdout".to_string()),
                (true, "oops from stderr".to_string()),
            ]
        );
    }

    #[test]
    fn splits_multiple_lines_inside_one_frame() {
        let body = frame(1, "first\nsecond\nthird\n");
        let lines = demux_log_stream(&body);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2].1, "third");
    }

    #[test]
    fn handles_tty_output_without_framing() {
        // TTY containers stream raw text; framing must not be assumed.
        let body = b"plain line one\nplain line two\n";
        let lines = demux_log_stream(body);

        assert_eq!(
            lines,
            vec![
                (false, "plain line one".to_string()),
                (false, "plain line two".to_string()),
            ]
        );
    }

    #[test]
    fn a_truncated_final_frame_does_not_panic() {
        // Header claims 100 bytes but only 5 follow — happens when a tail cuts
        // the stream mid-frame.
        let mut body = vec![1, 0, 0, 0];
        body.extend_from_slice(&100u32.to_be_bytes());
        body.extend_from_slice(b"short");

        let lines = demux_log_stream(&body);
        assert_eq!(lines, vec![(false, "short".to_string())]);
    }

    #[test]
    fn invalid_utf8_is_replaced_not_dropped() {
        let mut payload = b"before ".to_vec();
        payload.push(0xFF);
        payload.extend_from_slice(b" after\n");

        let mut body = vec![1, 0, 0, 0];
        body.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        body.extend_from_slice(&payload);

        let lines = demux_log_stream(&body);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].1.contains("before"));
        assert!(lines[0].1.contains("after"));
    }

    #[test]
    fn empty_body_yields_no_lines() {
        assert!(demux_log_stream(b"").is_empty());
    }

    #[test]
    fn blank_lines_are_skipped() {
        let body = frame(1, "one\n\n\ntwo\n");
        let lines = demux_log_stream(&body);

        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn detects_framing_correctly() {
        assert!(looks_multiplexed(&frame(1, "x")));
        assert!(looks_multiplexed(&frame(2, "x")));
        // Plain text: byte 0 is 'p', well above the stream-type range.
        assert!(!looks_multiplexed(b"plain text log line"));
        // Too short to hold a header.
        assert!(!looks_multiplexed(b"abc"));
        // Stream type 3 is not valid.
        assert!(!looks_multiplexed(&[3, 0, 0, 0, 0, 0, 0, 1, b'x']));
    }
}
