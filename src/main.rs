#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

mod band;
use band::{Band};

#[post("/", data= "<band>")]
fn create(band: Json<Band>) -> Json<Band> {
    band
}

#[get("/")]
fn read() -> JsonValue {
json!([
    "band 1",
    "band 2"
])
}

#[put("/<id>", data= "<band>")]
fn update(id: i32, band: Json<Band>) -> Json<Band> {
    band
}

#[delete("/<id>")]
fn delete(id: i32) -> JsonValue {
    json!({ "status": "ok" })
}

fn main() {
    rocket::ignite()
        .mount("/band", routes![create, update, delete])
        .mount("/bands", routes![read])
        .launch();
}