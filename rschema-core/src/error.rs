#[derive(Debug, thiserror::Error)]
pub enum RschemaError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
