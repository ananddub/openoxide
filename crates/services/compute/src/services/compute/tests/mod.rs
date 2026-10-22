mod application;
mod compose;
mod database;
mod engine;
mod preview;

use buffa::EnumValue;

use crate::api::connect::generated::openoxide::api::v1::{
    ApplicationBuildType, ApplicationSourceType, CreateApplicationRequest,
};
use crate::services::compute::application::ApplicationRecord;

pub(crate) use super::test_support::TestDb;

pub(crate) async fn create_application(
    db: &TestDb,
    name: &str,
    environment_id: i64,
    server_id: Option<i64>,
) -> ApplicationRecord {
    db.application()
        .create(CreateApplicationRequest {
            name: name.into(),
            environment_id,
            server_id,
            source_type: EnumValue::Known(ApplicationSourceType::Github),
            build_type: EnumValue::Known(ApplicationBuildType::Nixpacks),
            ..Default::default()
        })
        .await
        .unwrap()
}

pub(crate) fn assert_protocol<T: std::fmt::Debug>(result: sqlx::Result<T>, needle: &str) {
    match result {
        Err(sqlx::Error::Protocol(message)) => {
            assert!(
                message.contains(needle),
                "expected protocol error containing {needle:?}, got {message:?}"
            );
        }
        other => panic!("expected protocol error containing {needle:?}, got {other:?}"),
    }
}
