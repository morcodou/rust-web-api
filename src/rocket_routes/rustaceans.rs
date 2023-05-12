use crate::models::NewRustacean;
use crate::repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::serde::json::Json;

use crate::DbConn;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db_poll: DbConn) -> Result<Value, Custom<Value>> {
    db_poll
        .run(|conn| {
            RustaceanRepository::find_multiple(conn, 100)
                .map(|rustaceans| json!(rustaceans))
                .map_err(|er| Custom(Status::InternalServerError, json!("get_rustaceans error")))
        })
        .await
}

#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(db_poll: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>>{
    db_poll
        .run(move |conn| {
            RustaceanRepository::create(conn, new_rustacean.into_inner())
                .map(|rustacean| json!(rustacean))
                .map_err(|er| Custom(Status::InternalServerError, json!("create error")))
        })
        .await
}
