#[macro_export]
macro_rules! impl_builder_opts {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
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
