#[macro_use]
extern crate rocket;

mod request_models;
mod routes;

use rocket::http::Status;
use rocket::response::{content, status};
use rocket::Request;
use rocket_sync_db_pools::{database, diesel};

#[database("metrix_db")]
pub struct MetrixDatabaseConnection(diesel::PgConnection);

#[catch(404)]
fn general_not_found() -> content::Html<&'static str> {
    content::Html(r#"<p>404 Not Found</p>"#)
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            routes::get_metric, 
            routes::post_metric,
            routes::get_metric_history,
            routes::get_metric_series
            ])
        .register("/", catchers![general_not_found, default_catcher])
        .attach(MetrixDatabaseConnection::fairing())
}
