use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("serde yaml error: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("file io error: {0}")]
    FiloIo(#[from] io::Error),

    #[error("jwt_simple error: {0}")]
    Jwt(#[from] jwt_simple::Error),

    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}
