use crate::repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

use crate::DbConn;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db_poll: DbConn) -> Result<Value, Custom<Value>> {
    db_poll
        .run(|conn| {
            RustaceanRepository::find_multiple(conn, 100)
                .map(|rustaceans| json!(rustaceans))
                .map_err(|er| Custom(Status::InternalServerError, json!("error wrong")))
        })
        .await
}

// #[rocket::post("/rustaceans")]
// pub fn create_rustacean() {}
