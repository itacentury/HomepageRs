use rocket::form::validate::Contains;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::uri;
use site::rocket;

#[test]
fn test_index_status() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(site::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
}

#[test]
fn test_index_sections() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(site::index)).dispatch();
    let result_string = response.into_string().unwrap();
    assert!(result_string.contains("Projects"));
    assert!(result_string.contains("Experience"));
    assert!(result_string.contains("Education"));
    assert!(result_string.contains("Links"));
}
