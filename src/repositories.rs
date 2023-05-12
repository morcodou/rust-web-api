use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(connection: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(connection)
    }

    // pub fn find(connection: &PgConnection, id: i32) -> QueryResult<Rustacean> {
    //     rustaceans::table
    //         .find(id)
    //         .get_result(connection)
    // }

    pub fn create(
        connection: &mut PgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(connection)
    }

    // pub fn save(
    //     connection: &mut PgConnection,
    //     id: i32,
    //     rustacean: Rustacean,
    // ) -> QueryResult<Rustacean> {
    //     diesel::update(rustaceans::table.find(id))
    //         .set((
    //             rustaceans::email.eq(rustacean.email.to_owned()),
    //             rustaceans::name.eq(rustacean.name.to_owned()),
    //         ))
    //         .execute(connection)?;

    //     Self::find(connection, id)
    // }

    // pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
    //     diesel::delete(rustaceans::table.find(id)).execute(connection)
    // }
}
