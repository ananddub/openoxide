//! `gzip`/`gunzip` as streaming pipeline steps (`cmd | gzip`, `gunzip | cmd`).

use crate::exec::script::IntoCommand;

macro_rules! stream_codec_builder {
    ($name:ident, $binary:literal) => {
        #[doc = concat!("Builds a streaming `", $binary, "` step (stdin to stdout).")]
        #[derive(Default, Clone, Debug)]
        pub struct $name {
            args: Vec<String>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            #[doc = concat!("Force output to `", $binary, "` even if it is a terminal (`--force`).")]
            pub fn force(mut self) -> Self {
                self.args.push("--force".into());
                self
            }

            /// Accumulated argv (binary excluded).
            pub fn build_args(&self) -> Vec<String> {
                self.args.clone()
            }
        }

        impl IntoCommand for $name {
            fn build_str(&self) -> String {
                let mut parts = vec![$binary.to_string()];
                parts.extend(
                    self.args
                        .iter()
                        .map(|a| crate::exec::script::shell_single_quote(a)),
                );
                parts.join(" ")
            }
        }
    };
}

stream_codec_builder!(GzipBuilder, "gzip");
stream_codec_builder!(GunzipBuilder, "gunzip");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gzip_step_is_a_bare_command() {
        assert_eq!(GzipBuilder::new().build_str(), "gzip");
        assert_eq!(GunzipBuilder::new().force().build_str(), "gunzip '--force'");
    }
}
