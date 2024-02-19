#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error(transparent)]
    EnvError(#[from] dotenv::Error),

    #[error(transparent)]
    TeloxideError(#[from] teloxide::RequestError),

    #[error(transparent)]
    InMemStorageError(#[from] teloxide::dispatching::dialogue::InMemStorageError),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Errors>;
