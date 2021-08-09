#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod request_models;
mod routes;
mod insert_queue;

use crate::insert_queue::InsertQueueManager;
use dotenv::dotenv;
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
    initialize_utilities();
    initialize_background_processors();
    
    // everything else
    rocket::build()
    .mount("/", routes![
        routes::get_metric, 
        routes::post_metric,
        routes::get_metric_history,
        routes::get_metric_series,
        routes::get_metric_data_groupings,
        routes::get_metric_data_points_for_grouping
        ])
    .register("/", catchers![general_not_found, default_catcher])
    .attach(MetrixDatabaseConnection::fairing())
}

fn initialize_utilities() {
    pretty_env_logger::init();
    dotenv().ok();
    println!("{}", metrix_utils::get_header("API").as_str());
}

fn initialize_background_processors() {
    InsertQueueManager::process_queue_thread();
}
