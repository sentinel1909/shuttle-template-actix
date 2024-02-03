// src/lib/routes/not_found.rs

// dependencies
use actix_web::{HttpResponse, Responder};

// not found endpoint
#[tracing::instrument(name = "not_found")]
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().finish()
}