use std::sync::Arc;

use crate::{
    config::Config,
    domain::services::stats_service::StatsService,
    jobs::{JobRunner, startup::DataGenerationJob},
    presentation::{handlers::stats_handler::stats_routes, shared::app_state::AppState},
};
use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use anyhow::Result;
use env_logger::Env;

use super::{
    clickhouse::db_connection::db_connect, repositories::transfer_repo::ClickHouseTransferRepo,
};

pub struct AppDependencies {
    pub app_state: AppState,
}

impl AppDependencies {
    pub async fn init(config: &Config) -> Result<Self> {
        let clickhouse_client = db_connect(config).await?;

        let transfer_repo = Arc::new(ClickHouseTransferRepo::new(clickhouse_client));
        transfer_repo.create_table().await?;
        let stats_service = Arc::new(StatsService::new(transfer_repo.clone()));

        let data_gen_job = DataGenerationJob::new(config.data_generation_count, transfer_repo);
        let job_runner = JobRunner::new().add_job(data_gen_job);
        job_runner.run_all().await?;

        let app_state = AppState::new(stats_service);

        Ok(AppDependencies { app_state })
    }
}

pub async fn server(app_state: AppState, port: &str) -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let address = format!("0.0.0.0:{}", port);
    println!("App address: {}", &address);

    let app_state = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(stats_routes)
            .default_service(web::to(HttpResponse::MethodNotAllowed))
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
