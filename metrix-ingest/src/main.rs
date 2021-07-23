#[macro_use]
extern crate rocket;

mod request_models;

use crate::request_models::{
    MetricRequest, OptionsHistoryRequest, OptionsHistorySeriesRequest, OptionsRequest,
};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{json, Json, Value};
use rocket::{Build, Request, Rocket};

#[post("/metric", format = "json", data = "<metric>")]
fn post_metric(metric: Json<MetricRequest<'_>>) {}

#[get("/metric?<opt..>")]
fn get_metric(opt: OptionsRequest<'_>) {}

#[get("/metric-history?<opt..>")]
fn get_metric_history(opt: OptionsHistoryRequest<'_>) {}

#[get("/metric-history-series?<opt..>")]
fn get_metric_series(opt: OptionsHistorySeriesRequest<'_>) {}

#[catch(404)]
fn general_not_found() -> content::Html<&'static str> {
    content::Html(r#"<p>404 Not Found</p>"#)
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
