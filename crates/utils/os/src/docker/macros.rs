#[macro_export]
macro_rules! impl_builder_opts {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
            /// Execute this operation through the Docker Engine API instead of
            /// the default Docker CLI path.
            pub fn api(mut self) -> Self {
                self.args.api = true;
                self
            }
            /// Set the maximum number of retries for this command.
            pub fn retry(mut self, max_retries: u32) -> Self {
                self.args.retry_limit = Some(max_retries);
                self
            }

            /// Attach a cancellation token to gracefully abort long-running commands.
            pub fn cancel_with(mut self, token: tokio_util::sync::CancellationToken) -> Self {
                self.args.cancel_token = Some(token);
                self
            }
        }
    };
}

/// Builder options for commands that are intentionally CLI-only.
///
/// Keep `.api()` out of this implementation so unsupported operations cannot
/// accidentally opt into Engine API mode at compile time.
#[macro_export]
macro_rules! impl_cli_builder_opts {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
            pub fn retry(mut self, max_retries: u32) -> Self {
                self.args.retry_limit = Some(max_retries);
                self
            }

            pub fn cancel_with(mut self, token: tokio_util::sync::CancellationToken) -> Self {
                self.args.cancel_token = Some(token);
                self
            }
        }
    };
}
