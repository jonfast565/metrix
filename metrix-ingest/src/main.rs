#[macro_use]
extern crate rocket;

mod request_models;

use crate::request_models::{
    MetricQueryRequest, MetricPointQueryRequest, MetricRangeRequest, MetricInsertRequest
};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{json, Json, Value};
use rocket::{Build, Request, Rocket};

#[post("/metric", format = "json", data = "<metric>")]
fn post_metric(metric: Json<MetricInsertRequest<'_>>) {}

#[get("/metric?<opt..>")]
fn get_metric(opt: MetricQueryRequest<'_>) {}

#[get("/metric-point?<opt..>")]
fn get_metric_history(opt: MetricPointQueryRequest<'_>) {}

#[get("/metric-series?<opt..>")]
fn get_metric_series(opt: MetricRangeRequest<'_>) {}

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
