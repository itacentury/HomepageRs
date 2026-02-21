use actix_web::{App, test, web};
use std::time::Duration;

fn create_app_config() -> (
    web::Data<handlebars::Handlebars<'static>>,
    web::Data<site::github::RepoCache>,
    web::Data<site::config::AppConfig>,
) {
    let hb = web::Data::new(site::create_handlebars());
    let cache = web::Data::new(site::github::RepoCache::new(
        None,
        "test".to_string(),
        Duration::from_secs(300),
    ));
    let config = web::Data::new(site::config::AppConfig {
        personal: site::config::PersonalConfig {
            name: "Test User".to_string(),
            title: "Test Title".to_string(),
            meta_description: "Test description".to_string(),
            og_title: "Test OG Title".to_string(),
            og_description: "Test OG Description".to_string(),
        },
        github: site::config::GithubConfig {
            username: "test".to_string(),
            token: None,
            cache_ttl_secs: 300,
        },
        education: vec![site::config::Education {
            r#type: "Bachelor".to_string(),
            year: "2020-2024".to_string(),
            name: "CS".to_string(),
            place: "University".to_string(),
        }],
        experience: vec![site::config::Experience {
            r#type: "Fulltime".to_string(),
            year: "2024-Now".to_string(),
            name: "Developer".to_string(),
            place: "Company".to_string(),
        }],
        links: vec![site::config::Link {
            name: "GitHub".to_string(),
            link: "https://github.com/test".to_string(),
            linkname: "github.com/test".to_string(),
        }],
    });
    (hb, cache, config)
}

#[actix_web::test]
async fn test_index_status() {
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
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
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
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
async fn test_index_renders_config_data() {
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();

    assert!(body.contains("Test User"));
    assert!(body.contains("Test Title"));
    assert!(body.contains("Test OG Title"));
}

#[actix_web::test]
async fn test_not_found() {
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
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
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_github_fallback() {
    let (hb, cache, config) = create_app_config();
    let app = test::init_service(
        App::new()
            .app_data(hb)
            .app_data(cache)
            .app_data(config)
            .configure(site::app_config),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();

    assert!(body.contains("Visit my GitHub"));
}
