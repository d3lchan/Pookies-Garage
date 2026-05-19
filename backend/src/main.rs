#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;
#[derive]

#[get("/")]
fn index() -> &'static str{
    "Pookies Garage API"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
