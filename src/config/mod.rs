mod app_config;
mod status_code;

use anyhow::Result;
pub use app_config::AppConfig;
pub use status_code::{STATUS_CODE, code_init};

pub async fn config_init() -> Result<AppConfig> {
    code_init().await;
    AppConfig::new()
}
