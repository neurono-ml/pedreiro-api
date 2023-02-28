use std::fmt::Display;

#[derive(Debug)]
pub struct InternalServerError {
    error: anyhow::Error,
}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{error}", error=self.error)
    }
}

impl actix_web::error::ResponseError for InternalServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for InternalServerError {
    fn from(error: anyhow::Error) -> InternalServerError {
        InternalServerError { error: error }
    }
}