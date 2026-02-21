use actix_web::{HttpRequest, HttpResponse, web};
use handlebars::Handlebars;
use serde_json::json;

pub mod content;
pub mod github;

/// Create and configure a Handlebars registry with all project templates.
///
/// Terminates the process with an error message if any template fails to load,
/// since the application cannot serve pages without its templates.
pub fn create_handlebars() -> Handlebars<'static> {
    let mut hb = Handlebars::new();

    let templates = [
        ("index", "templates/index.html.hbs"),
        ("layout", "templates/layout.html.hbs"),
        ("nav", "templates/nav.html.hbs"),
        ("projects", "templates/projects.html.hbs"),
        ("education", "templates/education.html.hbs"),
        ("experience", "templates/experience.html.hbs"),
        ("links", "templates/links.html.hbs"),
        ("error/404", "templates/error/404.html.hbs"),
        ("error/500", "templates/error/500.html.hbs"),
    ];

    for (name, path) in templates {
        if let Err(e) = hb.register_template_file(name, path) {
            eprintln!("Fatal: failed to register template '{name}': {e}");
            std::process::exit(1);
        }
    }

    hb
}

/// Render the index page with all content sections.
pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    repo_cache: web::Data<github::RepoCache>,
) -> HttpResponse {
    let repos = repo_cache.get_repos().await;
    let github_unavailable = repos.is_none();

    let body = hb.render(
        "index",
        &json!({
            "repos": repos.unwrap_or_default(),
            "github_unavailable": github_unavailable,
            "education": content::get_education(),
            "experience": content::get_experience(),
            "links": content::get_links(),
        }),
    );

    match body {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => {
            eprintln!("Template render error: {e}");
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(
                    hb.render("error/500", &json!({}))
                        .unwrap_or_else(|_| "<h1>500 — Internal Server Error</h1>".into()),
                )
        }
    }
}

/// Return a simple health check response for container orchestration.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Render a 404 page for unmatched routes.
pub async fn not_found(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = hb
        .render("error/404", &json!({ "uri": req.uri().to_string() }))
        .unwrap_or_else(|_| "<h1>404 — Not Found</h1>".into());
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(body)
}

/// Configure actix-web app routes and services.
pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/health", web::get().to(health))
        .service(actix_files::Files::new("/", "./static"))
        .default_service(web::route().to(not_found));
}
