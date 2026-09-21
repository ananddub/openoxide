use crate::utils::CommandExecutor;
struct KubernetesConfig {
    endpoint: String,
    ca_cert: Vec<u8>,
    client_cert: Option<Vec<u8>>,
    client_key: Option<Vec<u8>>,
    token: Option<String>,
    namespace: String,
}
enum ContainerEngine {
    Docker(CommandExecutor),
    K8s(KubernetesConfig),
}
