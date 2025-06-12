use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::{transfer::Transfer, user_stats::UserStats};

use super::errors::TransferRepoError;

pub type TransferRepoResult<T> = Result<T, TransferRepoError>;

#[automock]
#[async_trait]
pub trait TransferRepoAbstract {
    async fn save_all(&self, transfers: &[Transfer]) -> TransferRepoResult<()>;
    async fn calculate_user_stats(&self) -> TransferRepoResult<Vec<UserStats>>;
}
