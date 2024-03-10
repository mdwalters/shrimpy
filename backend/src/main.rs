#[macro_use] extern crate rocket;
use rocket::serde::json::*;

#[get("/")]
fn index() -> Value {
    json!({
        "version": "0.1.0"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index
            ]
        )
}
