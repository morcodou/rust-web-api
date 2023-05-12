use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Associations};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = rustaceans)]
// #[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Rustacean))]
// #[belongs_to(Rustacean)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crates)]
// #[table_name = "crates"]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
