use actix_web::{HttpResponse, Responder, get, web};

use crate::{domain::services::errors::TransferError, presentation::shared::app_state::AppState};

pub fn stats_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/stats").service(get_all));
}

#[get("/get_all")]
async fn get_all(app_state: web::Data<AppState>) -> Result<impl Responder, TransferError> {
    let stats = app_state.stats_service.calculate_user_stats().await?;
    Ok(HttpResponse::Ok().json(stats))
}
