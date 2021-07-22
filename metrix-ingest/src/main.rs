#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::{content, status};
use rocket::{Build, Request, Rocket};

#[post("/metric")]
fn post_metric() {

}

#[get("/metric")]
fn get_metric() {
    
}

#[catch(404)]
fn general_not_found() -> content::Html<&'static str> {
    content::Html(
        r#"<p>404 Not Found</p>"#,
    )
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", routes![get_metric, post_metric])
        .register("/", catchers![general_not_found, default_catcher])
        .launch()
        .await
    {
        println!("App didn't launch: {}", e);
        drop(e);
    }
}
