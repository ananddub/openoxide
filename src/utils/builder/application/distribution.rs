use super::application::ApplicationBuilder;
use crate::utils::builder::spec::ApplicationSpec;
use crate::utils::exec::ExecResult;
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn distribute_image(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<String> {
        let Some(auth) = &spec.image_registry else {
            return Ok(spec.image.clone());
        };
        self.ctx.cancelled(cancel)?;
        self.ctx
            .docker
            .system()
            .login()
            .registry(&auth.registry)
            .username(&auth.username)
            .password(&auth.password)
            .run()
            .await?;
        let target = distribution_image(&spec.image, &auth.registry);
        let result = async {
            self.ctx.cancelled(cancel)?;
            if target != spec.image {
                self.ctx
                    .docker
                    .images()
                    .tag(spec.image.clone(), target.clone())
                    .run()
                    .await?;
            }
            self.ctx
                .docker
                .images()
                .push(target.clone())
                .cancel_with(cancel.clone())
                .run()
                .await?;
            Ok::<(), crate::utils::exec::ExecError>(())
        }
        .await;
        let logout = self
            .ctx
            .docker
            .system()
            .logout()
            .registry(&auth.registry)
            .run()
            .await;
        result?;
        logout?;
        Ok(target)
    }
}

fn distribution_image(image: &str, registry: &str) -> String {
    let registry = registry.trim().trim_end_matches('/');
    if registry.is_empty() || image.starts_with(&format!("{registry}/")) {
        image.to_owned()
    } else {
        format!("{registry}/{image}")
    }
}

#[cfg(test)]
mod tests {
    use super::distribution_image;

    #[test]
    fn keeps_an_already_qualified_image() {
        assert_eq!(
            distribution_image("registry.test/api:latest", "registry.test/"),
            "registry.test/api:latest"
        );
    }

    #[test]
    fn qualifies_a_local_image() {
        assert_eq!(
            distribution_image("api:latest", "registry.test"),
            "registry.test/api:latest"
        );
    }
}
