use crate::{
    domain::repositories::transfer_repo::TransferRepoAbstract,
    infrastructure::generator::{TransferGenConfig, TransferGenerator},
    jobs::Job,
};
use anyhow::Result;
use std::sync::Arc;
pub struct DataGenerationJob<T: TransferRepoAbstract> {
    count: usize,
    transfer_repo: Arc<T>,
}

impl<T: TransferRepoAbstract> DataGenerationJob<T> {
    pub fn new(count: usize, transfer_repo: Arc<T>) -> Self {
        Self {
            count,
            transfer_repo,
        }
    }
}

impl<T: TransferRepoAbstract> Job for DataGenerationJob<T> {
    async fn run(&self) -> Result<()> {
        println!("Starting data generation job...");

        let generator = TransferGenConfig::default();
        let transfers = generator.generate(self.count)?;

        println!("Generated {} transfers", transfers.len());

        self.transfer_repo.save_all(&transfers).await?;

        println!("Data generation job completed successfully");
        Ok(())
    }
}
