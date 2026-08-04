use tracing::error;

use crate::docker::api::DockerApi;
use crate::docker::types::ContainerId;

const FRAME_HEADER_LEN: usize = 8;
const STREAM_STDERR: u8 = 2;

pub async fn tail_container_logs(
    api: &DockerApi,
    container_id: &ContainerId,
    tail_lines: usize,
) -> Vec<String> {
    let path = format!("/containers/{container_id}/logs?stdout=true&stderr=true&tail={tail_lines}");

    let body = match api.get_raw(&path).await {
        Ok(body) => body,
        Err(error) => {
            error!(%error, %container_id, "could not read container logs");
            return Vec::new();
        }
    };

    demux_log_stream(&body)
        .into_iter()
        .map(|(_, line)| line)
        .collect()
}

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
        assert!(!looks_multiplexed(b"plain text log line"));
        assert!(!looks_multiplexed(b"abc"));
        assert!(!looks_multiplexed(&[3, 0, 0, 0, 0, 0, 0, 1, b'x']));
    }
}
