
fn main() {
    println!("{}", metrix_utils::get_header("Aggregation"));
    run_aggregation();
}

fn run_aggregation() {
    let database_url = metrix_utils::envs::get_database_url();
    // let groups_in_range = ;
    // let range_query = ;
    // let series = metrix_database::get_metric_series_history(&range_query, conn: &PgConnection)
}

