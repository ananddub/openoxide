use serde::de::DeserializeOwned;

/// Accumulates raw chunks from a streaming docker endpoint and yields whole
/// JSON documents as they complete.
///
/// The daemon streams stats as a series of JSON documents with no framing
/// between them. Newlines cannot be used as delimiters — the daemon
/// pretty-prints, so newlines appear *inside* a document too. Instead the
/// buffer is scanned for the longest prefix ending in `}` that parses, which is
/// what the CLI effectively does as well.
#[derive(Default)]
pub struct JsonAccumulator {
    buffer: String,
}

impl JsonAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a chunk and returns every document that is now complete.
    ///
    /// Bytes that do not yet form a document stay buffered for the next call.
    pub fn push<T: DeserializeOwned>(&mut self, chunk: &[u8]) -> Vec<T> {
        self.buffer.push_str(&String::from_utf8_lossy(chunk));

        let mut documents = Vec::new();

        // Repeatedly take the longest parseable prefix. Searching from the last
        // `}` backwards handles several documents arriving in one chunk.
        loop {
            let Some(document) = self.take_one() else {
                break;
            };
            documents.push(document);
        }

        documents
    }

    fn take_one<T: DeserializeOwned>(&mut self) -> Option<T> {
        let mut search_end = self.buffer.len();

        while let Some(brace) = self.buffer[..search_end].rfind('}') {
            let candidate = &self.buffer[..=brace];

            if let Ok(value) = serde_json::from_str::<T>(candidate) {
                self.buffer.drain(..=brace);
                return Some(value);
            }

            // Not parseable up to this brace — try an earlier one.
            search_end = brace;
        }

        None
    }

    /// Bytes currently held back waiting for the rest of a document.
    #[cfg(test)]
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn parses_a_complete_document() {
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(br#"{"cpu":1}"#);

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0]["cpu"], 1);
        assert_eq!(acc.buffered_len(), 0);
    }

    #[test]
    fn a_document_split_across_chunks_still_parses() {
        let mut acc = JsonAccumulator::new();

        let first: Vec<Value> = acc.push(br#"{"cpu":1,"mem":"#);
        assert!(first.is_empty());

        let second: Vec<Value> = acc.push(br#""high"}"#);
        assert_eq!(second.len(), 1);
        assert_eq!(second[0]["mem"], "high");
    }

    #[test]
    fn multiple_documents_in_one_chunk_parse_separately() {
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(br#"{"a":1}{"b":2}"#);

        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0]["a"], 1);
        assert_eq!(docs[1]["b"], 2);
    }

    #[test]
    fn newlines_inside_a_document_do_not_break_parsing() {
        // The daemon pretty-prints, so a newline is not a document boundary.
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(b"{\n  \"cpu\": 1,\n  \"mem\": \"x\"\n}");

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0]["cpu"], 1);
    }

    #[test]
    fn a_partial_document_yields_nothing_yet() {
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(br#"{"cpu":1,"mem":"#);

        assert!(docs.is_empty());
        assert!(acc.buffered_len() > 0);
    }

    #[test]
    fn a_nested_document_is_not_split_at_an_inner_brace() {
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(br#"{"outer":{"inner":1},"after":2}"#);

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0]["after"], 2);
        assert_eq!(docs[0]["outer"]["inner"], 1);
    }

    #[test]
    fn invalid_utf8_does_not_panic() {
        let mut acc = JsonAccumulator::new();
        let docs: Vec<Value> = acc.push(&[0xFF, 0xFE]);
        assert!(docs.is_empty());
    }
}
