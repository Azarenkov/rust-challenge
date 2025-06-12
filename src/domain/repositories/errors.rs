use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferRepoError {
    #[error("Database connection failed: {0}")]
    DatabaseConnectionError(String),
    #[error("Transfer not found with id: {id}")]
    TransferNotFound { id: String },
    #[error("Database query failed: {0}")]
    QueryError(String),
}

#[derive(Debug, Error)]
pub enum UserStatsRepoError {}
