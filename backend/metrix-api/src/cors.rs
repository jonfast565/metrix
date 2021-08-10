use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        _res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        _res.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        _res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        _res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}