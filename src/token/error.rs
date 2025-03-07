#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("[creation_failed] {0}")]
    Creation(#[source] anyhow::Error),

    #[error("[validation_failed] {0}")]
    Validation(#[source] anyhow::Error),

    #[error("[parsing_failed] {0}")]
    Parsing(#[source] anyhow::Error),

    #[error("[invalid_format] {0}")]
    InvalidFormat(#[source] anyhow::Error),

    #[error("[missing_claims] {0}")]
    MissingClaims(#[source] anyhow::Error),

    #[error("transparent")]
    Other(#[source] anyhow::Error),
}
