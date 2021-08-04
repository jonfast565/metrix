use std::env;

fn main() {
    println!("{}", metrix_utils::get_header("Aggregation"));
    run_aggregation();
}

fn run_aggregation() {
    let database_url = get_database_url();
    // let groups_in_range = ;
    // let range_query = ;
    // let series = metrix_database::get_metric_series_history(&range_query, conn: &PgConnection)
}

fn get_database_url() -> String {
    let database_url_key = "DATABASE_URL";
    match env::var(database_url_key) {
        Ok(val) => {
            return val;
        },
        Err(e) => panic!("DATABASE_URL not passed to the application.")
    }
}

