use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InternalServerError {
    error_message: String,
}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{error}", error=self.error_message)
    }
}

impl actix_web::error::ResponseError for InternalServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for InternalServerError {
    fn from(error: anyhow::Error) -> InternalServerError {
        let error_message = format!("{error:?}");
        InternalServerError { error_message }
    }
}

impl From<serde_json::Error> for InternalServerError {
    fn from(error: serde_json::Error) -> InternalServerError {
        let error_message = format!("{error:?}");
        InternalServerError { error_message}
    }
}