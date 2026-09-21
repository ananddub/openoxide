//! Minimal parser for `rsync --info=progress2` byte telemetry.
//!
//! `--info=progress2` rewrites a single status line on the stream using `\r`,
//! of the shape:
//!
//! ```text
//! 344,244  12%    4.10MB/s    0:00:00  (xfr#1, ir-chk=1001/1005)
//! ```
//!
//! The first field is the cumulative number of bytes transferred so far (may
//! carry thousands separators), followed by the percentage of total work. The
//! migration SYNC phase records this into `migration_objects.bytes`, so the
//! parser is deliberately tolerant: any line it cannot understand yields
//! `None` rather than an error.

/// One parsed progress sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RsyncProgress {
    /// Cumulative bytes transferred so far.
    pub transferred_bytes: u64,
    /// Percentage of total work (0..=100) when rsync reports it.
    pub percent: Option<u8>,
}

/// Parse a single `--info=progress2` field/line. Returns `None` when the input
/// is not a recognisable progress line.
pub fn parse_progress2(line: &str) -> Option<RsyncProgress> {
    // A chunk may contain several `\r`-separated samples plus a trailing `\n`.
    // Report the last well-formed sample in the chunk.
    let mut best = None;
    for part in line.split(['\r', '\n']) {
        if let Some(p) = parse_sample(part) {
            best = Some(p);
        }
    }
    best
}

fn parse_sample(s: &str) -> Option<RsyncProgress> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let mut fields = s.split_whitespace();
    let bytes_tok = fields.next()?;

    // Strip thousands separators (`,` in most locales) and any stray `_`.
    let digits: String = bytes_tok.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() || digits.len() != bytes_tok.replace([',', '_'], "").len() {
        // The first token was not a plain (optionally grouped) number.
        return None;
    }
    let transferred_bytes = digits.parse::<u64>().ok()?;

    let percent = fields.find_map(|f| f.strip_suffix('%')).and_then(|p| p.parse::<u8>().ok());

    Some(RsyncProgress {
        transferred_bytes,
        percent,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_grouped_bytes_and_percent() {
        let p = parse_progress2(
            "344,244  12%    4.10MB/s    0:00:00  (xfr#1, ir-chk=1001/1005)",
        )
        .expect("should parse");
        assert_eq!(p.transferred_bytes, 344244);
        assert_eq!(p.percent, Some(12));
    }

    #[test]
    fn parses_plain_bytes_without_percent() {
        let p = parse_progress2("9983440266 100%  226.52MB/s  0:00:42")
            .expect("should parse");
        assert_eq!(p.transferred_bytes, 9983440266);
        assert_eq!(p.percent, Some(100));
    }

    #[test]
    fn takes_last_sample_in_a_carriage_return_chunk() {
        let chunk = "1000  1%\r5000  5%\r";
        let p = parse_progress2(chunk).expect("should parse");
        assert_eq!(p.transferred_bytes, 5000);
        assert_eq!(p.percent, Some(5));
    }

    #[test]
    fn rejects_non_progress_lines() {
        assert!(parse_progress2("sending incremental file list").is_none());
        assert!(parse_progress2("rsync error: some files vanished").is_none());
        assert!(parse_progress2("").is_none());
        assert!(parse_progress2("   ").is_none());
    }
}
