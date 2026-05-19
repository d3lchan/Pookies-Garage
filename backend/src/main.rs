#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Part {
id: i32,
name: String,
category: String,
price_cents: i32,
in_stock: bool,
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
fn get_parts_by_category(category: &str) -> Json<Vec<Part>
#[get("/")]
fn index() -> &'static str{
    "Pookies Garage API"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,get_parts])
}
