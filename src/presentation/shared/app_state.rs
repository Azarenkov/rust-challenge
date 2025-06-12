use std::sync::Arc;

use crate::{
    domain::services::stats_service::StatsService,
    infrastructure::repositories::transfer_repo::ClickHouseTransferRepo,
};

pub struct AppState {
    pub stats_service: Arc<StatsService<ClickHouseTransferRepo>>,
}

impl AppState {
    pub fn new(stats_service: Arc<StatsService<ClickHouseTransferRepo>>) -> Self {
        Self { stats_service }
    }
}
