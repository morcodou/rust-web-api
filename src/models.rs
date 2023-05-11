use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable)]
struct Rustacean {
    id: i32,
    name: String,
    email: String,
    create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "rustaceans"]
struct NewRustacean {
    name: String,
    email: String,
}

#[derive(Queryable, Associations)]
#[belongs_to(Rustacean)]
struct Crate {
    id: i32,
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
    create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "crates"]
struct NewCrate {
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
}
