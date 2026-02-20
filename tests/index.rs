use actix_web::{App, test, web};

fn create_app_config() -> web::Data<handlebars::Handlebars<'static>> {
    web::Data::new(site::create_handlebars())
}

#[actix_web::test]
async fn test_index_status() {
    let hb = create_app_config();
    let app = test::init_service(App::new().app_data(hb).configure(site::app_config)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/html");
}

#[actix_web::test]
async fn test_index_sections() {
    let hb = create_app_config();
    let app = test::init_service(App::new().app_data(hb).configure(site::app_config)).await;

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
    let hb = create_app_config();
    let app = test::init_service(App::new().app_data(hb).configure(site::app_config)).await;

    let req = test::TestRequest::get().uri("/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/html");

    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();
    assert!(body.contains("/nonexistent"));
}

#[actix_web::test]
async fn test_health() {
    let hb = create_app_config();
    let app = test::init_service(App::new().app_data(hb).configure(site::app_config)).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}
