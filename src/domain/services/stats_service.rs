use std::sync::Arc;

use crate::domain::{
    entities::user_stats::UserStats, repositories::transfer_repo::TransferRepoAbstract,
};

use super::errors::TransferError;

pub type StatsServiceResult<T> = Result<T, TransferError>;

pub struct StatsService<T>
where
    T: TransferRepoAbstract,
{
    transfer_repo: Arc<T>,
}

impl<T> StatsService<T>
where
    T: TransferRepoAbstract,
{
    pub fn new(transfer_repo: Arc<T>) -> Self {
        Self { transfer_repo }
    }

    pub async fn calculate_user_stats(&self) -> StatsServiceResult<Vec<UserStats>> {
        let stats = self.transfer_repo.calculate_user_stats().await?;
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::user_stats::UserStats,
        repositories::{errors::TransferRepoError, transfer_repo::MockTransferRepoAbstract},
    };
    use std::sync::Arc;

    fn create_test_stats() -> Vec<UserStats> {
        vec![
            UserStats::new("0x123".to_string(), 1000.0, 1.0, 1.5, 500.0), // Profitable
            UserStats::new("0x456".to_string(), 800.0, 1.2, 1.1, 400.0),  // Loss
            UserStats::new("0x789".to_string(), 1200.0, 1.1, 1.6, 600.0), // Profitable
        ]
    }

    #[actix_web::test]
    async fn test_calculate_user_stats_success() {
        let mut mock_repo = MockTransferRepoAbstract::new();

        mock_repo
            .expect_calculate_user_stats()
            .times(1)
            .returning(move || Ok(create_test_stats()));

        let service = StatsService::new(Arc::new(mock_repo));
        let result = service.calculate_user_stats().await;

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.len(), 3);
        assert_eq!(stats[0].address, "0x123");
    }

    #[actix_web::test]
    async fn test_calculate_user_stats_repo_error() {
        let mut mock_repo = MockTransferRepoAbstract::new();

        mock_repo
            .expect_calculate_user_stats()
            .times(1)
            .returning(|| Err(TransferRepoError::QueryError("DB Error".to_string())));

        let service = StatsService::new(Arc::new(mock_repo));
        let result = service.calculate_user_stats().await;

        assert!(result.is_err());
    }
}
