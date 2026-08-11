// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!! KEEP THIS IN SYNC WITH `html-macro/src/generate.rs` (escape) !!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

pub fn escape_to_string(input: &str, output: &mut String) {
    for b in input.bytes() {
        match b {
            b'&' => output.push_str("&amp;"),
            b'<' => output.push_str("&lt;"),
            b'>' => output.push_str("&gt;"),
            b'"' => output.push_str("&quot;"),
            // SAFETY: matched bytes are all ASCII (single-byte), so the
            // remaining bytes of any multi-byte UTF-8 sequence are untouched.
            _ => unsafe { output.as_mut_vec().push(b) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::escape_to_string;

    #[test]
    fn it_works() {
        let mut s = String::new();
        escape_to_string("<script>launchMissiles()</script>", &mut s);
        assert_eq!(s, "&lt;script&gt;launchMissiles()&lt;/script&gt;");
    }

    #[test]
    fn ampersand() {
        let mut s = String::new();
        escape_to_string("a & b", &mut s);
        assert_eq!(s, "a &amp; b");
    }

    #[test]
    fn quotes() {
        let mut s = String::new();
        escape_to_string(r#"say "hello""#, &mut s);
        assert_eq!(s, "say &quot;hello&quot;");
    }

    #[test]
    fn unicode_passthrough() {
        let mut s = String::new();
        escape_to_string("नमस्ते 🌍", &mut s);
        assert_eq!(s, "नमस्ते 🌍");
    }
}
