pub mod app_config;
pub mod status_code;

use crate::error::app_error::AppError;
use anyhow::Result;
use app_config::AppConfig;
use status_code::code_init;

#[allow(unused)]
pub async fn config_init() -> Result<AppConfig, AppError> {
    code_init().await;
    AppConfig::new()
}
