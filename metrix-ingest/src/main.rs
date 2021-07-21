#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![]).launch().await;
}
