#[macro_use] extern crate rocket;
use rocket::serde::json::*;

#[get("/")]
fn index() -> Value {
    json!({
        "version": "0.1.0"
    })
}

#[get("/manifest.json")]
fn manifest() -> Value {
    json!({
        "name": "Shrimpy",
        "short_name": "Shrimpy",
        "start_url": "/",
        "display": "standalone",
        "background_color": "#592f28",
        "theme_color": "#ff401f",
        "icons": [],
        "share_target": {}
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        manifest
    ])
}
