use actix_web::{App, HttpServer, web};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hb = site::create_handlebars();
    let hb_data = web::Data::new(hb);

    dotenvy::dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN").ok();
    if token.is_none() {
        eprintln!("Warning: GITHUB_TOKEN not set — projects section will show fallback");
    }
    let repo_cache = web::Data::new(site::github::RepoCache::new(
        token,
        Duration::from_secs(300),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(hb_data.clone())
            .app_data(repo_cache.clone())
            .configure(site::app_config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
