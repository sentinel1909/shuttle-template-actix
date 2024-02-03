// tests/api/health_check.rs

// dependencies
use actix_web::{http::header::ContentType, test, App};
use actix_web_template_lib::routes::health_check;

#[actix_web::test]
async fn health_check_works() {
  let app = test::init_service(App::new().service(health_check)).await;
  let req = test::TestRequest::default()
      .insert_header(ContentType::plaintext())
      .to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());
}

