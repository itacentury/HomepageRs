use actix_web::middleware::DefaultHeaders;
use actix_web::{App, HttpServer, web};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hb = site::create_handlebars();
    let hb_data = web::Data::new(hb);

    dotenvy::dotenv().ok();

    let config = match site::config::load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Fatal: failed to load configuration: {e}");
            std::process::exit(1);
        }
    };

    let token = config.github.token.clone();
    if token.is_none() {
        eprintln!("Warning: GitHub token not set — projects section will show fallback");
    }

    let repo_cache = web::Data::new(site::github::RepoCache::new(
        token,
        config.github.username.clone(),
        Duration::from_secs(config.github.cache_ttl_secs),
    ));

    let app_config = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                    .add((
                        "Content-Security-Policy",
                        "default-src 'self'; \
                         style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdnjs.cloudflare.com; \
                         font-src https://fonts.gstatic.com https://cdnjs.cloudflare.com; \
                         img-src 'self' data:; \
                         script-src 'self'",
                    )),
            )
            .app_data(hb_data.clone())
            .app_data(repo_cache.clone())
            .app_data(app_config.clone())
            .configure(site::app_config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
