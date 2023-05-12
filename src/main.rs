use rocket_sync_db_pools::{ diesel::PgConnection, database};

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod models;
mod repositories;
mod schema;
mod rocket_routes;

#[database("postgres")]
pub struct DbConn(PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![
        rocket_routes::rustaceans::get_rustaceans
        // rocket_routes::rustaceans::create_rustacean,
    ])
    .attach(DbConn::fairing())
    .launch().await;
}
