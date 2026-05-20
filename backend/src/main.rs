#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[derive(Serialize)]
struct Part {
id: i32,
name: String,
category: String,
price_cents: i32,
in_stock: bool,
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name:"Add CORS headers",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        }
}
#[get("/parts")]
fn get_parts() -> Json<Vec<Part>> {
    let parts = vec![
        Part {
            id: 1,
            name:String::from("Vesrah Ceramic Brake Pads"),
            category: String::from("Handling"),
            price_cents: 3499,
            in_stock: true,
        },
        Part {
            id: 1,
            name:String::from("LiquiMoly 10W40 1L engine oil"),
            category: String::from("Engine"),
            price_cents: 1299,
            in_stock: true,
        },
        Part {
            id: 1,
            name:String::from("R1250GS Driveshaft"),
            category: String::from("Drivetrain"),
            price_cents: 8499,
            in_stock: true,
        },
    ];
    Json(parts)
}

#[get("/parts/<category>")]
fn get_parts_by_category(category: &str) -> Json<Vec<Part>>{
    let parts = vec![
        Part {
            id: 1,
            name:String::from("Vesrah Ceramic Brake Pads"),
            category: String::from("Handling"),
            price_cents: 3499,
            in_stock: true,
        },
        Part {
            id: 1,
            name:String::from("LiquiMoly 10W40 1L engine oil"),
            category: String::from("Engine"),
            price_cents: 1299,
            in_stock: true,
        },
        Part {
            id: 1,
            name:String::from("R1250GS Driveshaft"),
            category: String::from("Drivetrain"),
            price_cents: 8499,
            in_stock: true,
        },
    ];
    let filtered_parts = parts.into_iter().filter(|part| part.category.to_lowercase() == category.to_lowercase()).collect();
    Json(filtered_parts)
}
#[get("/")]
fn index() -> &'static str{
    "Pookies Garage API"
}
#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount("/", routes![index,get_parts,get_parts_by_category])
}
