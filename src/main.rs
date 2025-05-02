use rocket_dyn_templates::{Template, context};
use rocket::{fs::{relative, FileServer}, Request};

use serde_json::json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { 
        name: "Julian",
        education: get_education_vec(),
        experience: get_experience_vec(),
    })
}

fn get_education_vec() -> Vec<serde_json::Value> {
    let education_vec = vec![
        json!({
            "type": "bachelor's degree",
            "year": "21/10-25/04",
            "name": "computer science",
            "place": "@ University of Augsburg",
        }),
        json!({
            "type": "vocational training",
            "year": "18/09-21/01",
            "name": "IT specialist",
            "place": "@ ESG Elektroniksystem- und Logistik GmbH",
        }),
    ];

    education_vec
}

fn get_experience_vec() -> Vec<serde_json::Value> {
    let experience_vec = vec![
        json!({ 
            "type": "fulltime employment", 
            "year": "25/05-...", 
            "name": "embedded software developer",
            "place": "@ WashTec Cleaning Technology GmbH",
        }),
        json!({ 
            "type": "working student", 
            "year": "25/01-25/03", 
            "name": "software developer",
            "place": "@ Hensoldt AG",
        }),
        json!({ 
            "type": "working student", 
            "year": "21/11–24/12", 
            "name": "software developer",
            "place": "@ ESG Elektroniksystem- und Logistik GmbH",
        }),
        json!({ 
            "type": "fulltime employment", 
            "year": "21/01-21/09", 
            "name": "software tester",
            "place": "@ ESG Elektroniksystem- und Logistik GmbH",
        }),
    ];

    experience_vec
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
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .register("/", catchers![internal_error, not_found])
        .attach(Template::fairing())
}
