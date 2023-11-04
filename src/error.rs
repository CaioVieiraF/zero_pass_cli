#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("ParseLangError")]
    ParseLangError,

    #[error(transparent)]
    ZeroPassError(#[from] zero_pass_backend::prelude::Error),
}
