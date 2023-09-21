use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use error_stack::Report;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, derive_more::Display)]
pub enum ResErr {
    #[display("Internal Server Error {message}")]
    InternalServerError { message: String },
    #[display("Unauthorized Error {message}")]
    UnauthorizedError { message: String },
}

impl actix_web::ResponseError for ResErr {
    fn status_code(&self) -> StatusCode {
        match self {
            ResErr::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ResErr::UnauthorizedError { .. } => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ResErr::InternalServerError { message } => {
                HttpResponse::InternalServerError().json(message)
            }
            ResErr::UnauthorizedError { message } => HttpResponse::Unauthorized().json(message),
        }
    }
}

impl<C> From<error_stack::Report<C>> for ResErr {
    fn from(report: Report<C>) -> Self {
        log::error!("{report:#?}");
        Self::InternalServerError {
            message: report.to_string(),
        }
    }
}
