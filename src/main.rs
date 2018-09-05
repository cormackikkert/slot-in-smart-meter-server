#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate influent;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket::Rocket;
use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;
use influent::create_client;
use influent::client::{Client, Credentials};
use influent::measurement::{Measurement, Value};


#[get("/")]
fn index() -> &'static str {
    "GG"
}

fn is_device_id_valid(device_id: u32) -> bool {
    device_id != 0
}

#[derive(Deserialize)]
struct Reading {
    consumption: i64,
    device_id: u32,
}

#[post("/dapi/reading", format = "application/json", data = "<reading>")]
fn dapi_put(reading: Json<Reading>) -> Result<&'static str, Failure> {
    let device_id = reading.0.device_id;
    if !is_device_id_valid(device_id) {
        Err(Failure(Status::Forbidden))
    } else {
        println!("Got reading for id={}, consumption={}",
                 device_id,
                 reading.0.consumption);

        let credentials = Credentials {
            username: "gobwas",
            password: "xxx",
            database: "smartmeter",
        };
        let hosts = vec!["http://localhost:8086"];
        let client = create_client(credentials, hosts);

        let mut measurement = Measurement::new("consumption");
        measurement.add_field("device_id", Value::Integer(device_id.into()));
        measurement.add_field("consumption", Value::Integer(reading.0.consumption));

        client.write_one(measurement, None).expect("Write result");

        let turn_on = true;
        if turn_on {
            Ok("ON ")
        } else {
            Ok("OFF")
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index, dapi_put])
}

fn main() {
    // prepare client
    rocket().launch();
}
