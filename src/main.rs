// src/main.rs

// dependencies
use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use shuttle_actix_web::ShuttleActixWeb;
use tracing::info;

// health_check endpoint
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(health_check);
    };
    info!("Health check endpoint is at: /health_check");
    Ok(config.into())
}
