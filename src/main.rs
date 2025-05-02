use rocket::Request;
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { name: "Julian" })
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![internal_error, not_found])
        .attach(Template::fairing())
}
