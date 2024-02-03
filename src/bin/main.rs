// src/main.rs

// dependencies
use actix_web::web::ServiceConfig;
use actix_web_template_lib::startup;
use shuttle_actix_web::ShuttleActixWeb;

// main function, annotated for Shuttle
#[shuttle_runtime::main] 
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    startup().await
}