use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hb = site::create_handlebars();
    let hb_data = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
            .app_data(hb_data.clone())
            .configure(site::app_config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
