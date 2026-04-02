use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Failed to connect to postagent server.")]
    ConnectionFailed,

    #[error("{0}")]
    ApiError(String),

    #[error("Auth not found for \"{project}\". Run: postagent auth {project}")]
    AuthNotFound { project: String },

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Error: API key cannot be empty.")]
    EmptyApiKey,

    #[error("Error: Permission denied. Check directory permissions.")]
    PermissionDenied,

    #[error("Aborted.")]
    Aborted,

    #[error("HTTP {status} {status_text}\n{body}")]
    HttpStatus {
        status: u16,
        status_text: String,
        body: String,
    },
}
