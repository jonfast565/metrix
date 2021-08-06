use std::env;

pub fn get_database_url() -> String {
    let database_url_key = "DATABASE_URL";
    match env::var(database_url_key) {
        Ok(val) => {
            return val;
        },
        Err(e) => panic!("DATABASE_URL not passed to the application. {}", e)
    }
}