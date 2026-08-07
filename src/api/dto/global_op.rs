use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContainerActionDto {
    Start,
    Stop,
    Restart,
    Kill,
    Pause,
    Unpause,
}

#[derive(Debug, Clone, Deserialize, Object, Validate)]
pub struct ContainerActionRequestDto {
    pub action: ContainerActionDto,
}

#[derive(Debug, Clone, Deserialize, Object, Validate)]
pub struct ContainerRemoveRequestDto {
    #[oai(default)]
    pub force: bool,
    #[oai(default)]
    pub volumes: bool,
}

#[derive(Debug, Clone, Deserialize, Object, Validate)]
pub struct DockerPruneRequestDto {
    #[oai(default)]
    pub containers: bool,
    #[oai(default)]
    pub images: bool,
    #[oai(default)]
    pub networks: bool,
    #[oai(default)]
    pub volumes: bool,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct DockerActionResponseDto {
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Object)]
pub struct DockerPruneResponseDto {
    pub operations: Vec<DockerActionResponseDto>,
}
