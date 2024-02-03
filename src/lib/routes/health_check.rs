// src/lie/routes/health_check.rs

// dependencies
use actix_web::{get, HttpResponse, Responder};

// health_check endpoint
#[tracing::instrument(name = "health_check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}