use rocket::{
    Request,
    fs::{FileServer, relative},
};
use rocket_dyn_templates::{Template, context};

#[macro_use]
extern crate rocket;

pub mod content;

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            education: content::get_education_vec(),
            experience: content::get_experience_vec(),
            links: content::get_links_vec(),
        },
    )
}

#[catch(500)]
fn internal_error() -> Template {
    Template::render("error/500", context! {})
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .register("/", catchers![internal_error, not_found])
        .attach(Template::fairing())
}
