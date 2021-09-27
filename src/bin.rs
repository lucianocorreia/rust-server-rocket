#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> Json<bool> {
    Json(true)
}
