// src/main.rs

// dependencies
use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

// health_check endpoint
#[tracing::instrument(name = "health_check")]
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Healthy")
}

// not found endpoint
#[tracing::instrument(name = "not_found")]
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json("Not Found")
}

// main function, annotated for Shuttle
#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // set up tracing, use tracing::fmt subscriber to write to stdout
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    tracing::info!("Tracing initialized");

    let config = move |cfg: &mut ServiceConfig| {
        // CORS configuration is preset to permissive, change this in real life!!
        let cors = Cors::permissive();

        // configure the service, under the /api scope, with tracing, CORS, the health_check endpoint, and a default not_found endpoint
        cfg.service(
            web::scope("/api")
                .wrap(cors)
                .wrap(TracingLogger::default())
                .service(health_check)
                .default_service(web::route().to(not_found)),
        );
    };

    // start the server
    tracing::info!("Starting server");
    Ok(config.into())
}
