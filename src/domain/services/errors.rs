use thiserror::Error;

use crate::domain::repositories::errors::TransferRepoError;

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] TransferRepoError),
}
