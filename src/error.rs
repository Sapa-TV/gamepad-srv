use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Gilrs library error: {0}")]
    Gilrs(#[from] gilrs::Error),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Skin error: {0}")]
    Skin(String),

    #[error("Internal error: {0}")]
    #[allow(dead_code)]
    Internal(String),

    #[error("Server error: {0}")]
    Server(String),
}

pub type AppResult<T> = Result<T, AppError>;
