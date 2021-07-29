pub mod time;
pub mod rocket;

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
