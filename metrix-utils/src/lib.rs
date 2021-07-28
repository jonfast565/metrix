use chrono::NaiveDateTime;
use std::env;

pub fn parse_iso_date_string(s: String) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.fZ").unwrap()
}

pub fn parse_iso_date_str(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.fZ").unwrap()
}

pub fn get_header(application_name: &str) -> String {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    format!(
r#"
    __  ___     __       _     
   /  |/  /__  / /______(_)  __
  / /|_/ / _ \/ __/ ___/ / |/_/
 / /  / /  __/ /_/ /  / />  <  
/_/  /_/\___/\__/_/  /_/_/|_|  
--------------------------------
Application: {}
Author(s): {}
Version: {}
"#,
        application_name,
        authors,
        version
    )
}
