use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::domain::{
    repositories::errors::TransferRepoError::{
        DatabaseConnectionError, QueryError, TransferNotFound,
    },
    services::errors::TransferError,
};

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}

impl ResponseError for TransferError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            TransferError::RepositoryError(transfer_repo_error) => match transfer_repo_error {
                DatabaseConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                TransferNotFound { id: _ } => StatusCode::NOT_FOUND,
                QueryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response = ApiError::new(self.to_string(), self.status_code().into());
        HttpResponse::build(self.status_code()).json(response)
    }
}
