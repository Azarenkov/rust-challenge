use crate::domain::repositories::errors::TransferRepoError;
use clickhouse::error::Error;

impl From<Error> for TransferRepoError {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidParams(error) => {
                TransferRepoError::QueryError(format!("Invalid parameters: {}", error))
            }
            Error::Network(error) => {
                TransferRepoError::DatabaseConnectionError(format!("Network error: {}", error))
            }
            Error::Compression(error) => {
                TransferRepoError::QueryError(format!("Compression error: {}", error))
            }
            Error::Decompression(error) => {
                TransferRepoError::QueryError(format!("Decompression error: {}", error))
            }
            Error::RowNotFound => TransferRepoError::TransferNotFound {
                id: "unknown".to_string(),
            },
            Error::SequenceMustHaveLength => {
                TransferRepoError::QueryError("Sequence must have length".to_string())
            }
            Error::DeserializeAnyNotSupported => {
                TransferRepoError::QueryError("Deserialization not supported".to_string())
            }
            Error::NotEnoughData => TransferRepoError::QueryError("Not enough data".to_string()),
            Error::InvalidUtf8Encoding(utf8_error) => {
                TransferRepoError::QueryError(format!("UTF-8 encoding error: {}", utf8_error))
            }
            Error::InvalidTagEncoding(tag) => {
                TransferRepoError::QueryError(format!("Invalid tag encoding: {}", tag))
            }
            Error::VariantDiscriminatorIsOutOfBound(value) => TransferRepoError::QueryError(
                format!("Variant discriminator out of bound: {}", value),
            ),
            Error::Custom(msg) => TransferRepoError::QueryError(format!("Custom error: {}", msg)),
            Error::BadResponse(msg) => {
                TransferRepoError::DatabaseConnectionError(format!("Bad response: {}", msg))
            }
            Error::TimedOut => {
                TransferRepoError::DatabaseConnectionError("Connection timed out".to_string())
            }
            Error::Unsupported(msg) => {
                TransferRepoError::QueryError(format!("Unsupported operation: {}", msg))
            }
            Error::Other(error) => TransferRepoError::QueryError(format!("Other error: {}", error)),
            _ => TransferRepoError::QueryError("Unknown ClickHouse error".to_string()),
        }
    }
}
