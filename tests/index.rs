use actix_web::{App, test, web};
use std::time::Duration;

fn create_app_config() -> (
    web::Data<handlebars::Handlebars<'static>>,
    web::Data<site::github::RepoCache>,
) {
    let hb = web::Data::new(site::create_handlebars());
    let cache = web::Data::new(site::github::RepoCache::new(
        None,
        "test".to_string(),
        Duration::from_secs(300),
    ));
    (hb, cache)
}

#[actix_web::test]
async fn test_index_status() {
    let (hb, cache) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/html");
}

#[actix_web::test]
async fn test_index_sections() {
    let (hb, cache) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();

    assert!(body.contains("Projects"));
    assert!(body.contains("Experience"));
    assert!(body.contains("Education"));
    assert!(body.contains("Links"));
}

#[actix_web::test]
async fn test_not_found() {
    let (hb, cache) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/html");

    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();
    assert!(body.contains("/nonexistent"));
}

#[actix_web::test]
async fn test_health() {
    let (hb, cache) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_github_fallback() {
    let (hb, cache) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();

    assert!(body.contains("Visit my GitHub"));
}
