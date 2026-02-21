use actix_web::middleware::DefaultHeaders;
use actix_web::{App, HttpServer, web};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hb = site::create_handlebars();
    let hb_data = web::Data::new(hb);

    dotenvy::dotenv().ok();

    let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
    if token.is_none() {
        eprintln!("Warning: GITHUB_TOKEN not set — projects section will show fallback");
    }

    let username = match std::env::var("GITHUB_USERNAME") {
        Ok(u) if !u.is_empty() => u,
        _ => {
            eprintln!("Fatal: GITHUB_USERNAME environment variable is required");
            std::process::exit(1);
        }
    };

    let cache_ttl = std::env::var("CACHE_TTL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(300u64);

    let repo_cache = web::Data::new(site::github::RepoCache::new(
        token,
        username,
        Duration::from_secs(cache_ttl),
    ));

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
            .configure(site::app_config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
