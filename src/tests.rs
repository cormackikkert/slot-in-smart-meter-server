use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn hello_world() {
	let client = Client::new(rocket()).expect("valid rocket instance");
	let mut response = client.get("/").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.body_string(), Some("GG".into()));
}

#[test]
fn invalid_device_id() {
	let client = Client::new(rocket()).expect("valid rocket instance");
	let response = client.get("/dapi/put/0").dispatch();
	assert_eq!(response.status(), Status::Forbidden);
}
