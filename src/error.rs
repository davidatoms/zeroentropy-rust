use thiserror::Error;

/// Result type for ZeroEntropy operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the ZeroEntropy SDK
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned an error status code
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Bad request (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Authentication error (401)
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Permission denied (403)
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Resource not found (404)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Conflict (409) - resource already exists
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Unprocessable entity (422)
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Internal server error (500+)
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    /// Failed to serialize/deserialize JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid API key
    #[error("Invalid API key: API key must be provided either via constructor or ZEROENTROPY_API_KEY environment variable")]
    InvalidApiKey,

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Base64 decoding error
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
}

impl Error {
    /// Create an API error from response status and message
    pub fn from_status(status: u16, message: String) -> Self {
        match status {
            400 => Error::BadRequest(message),
            401 => Error::AuthenticationError(message),
            403 => Error::PermissionDenied(message),
            404 => Error::NotFound(message),
            409 => Error::Conflict(message),
            422 => Error::UnprocessableEntity(message),
            429 => Error::RateLimitExceeded(message),
            500..=599 => Error::InternalServerError(message),
            _ => Error::Api { status, message },
        }
    }
}
