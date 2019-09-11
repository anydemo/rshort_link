pub mod tiny_link;
pub mod user;

use std::ops::Deref;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use tiny_link::{NewTinyLink, TinyLink};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn create_tiny_link(link: NewTinyLink, pool: &PgPool) -> Result<(), &'static str> {
    TinyLink::insert(link, get_conn(pool)?.deref()).map(|_|()).map_err(|_| "Error inserting tiny_link")
}

pub fn find_tiny_link(id: &str, pool: &PgPool) -> Result<TinyLink, &'static str> {
    TinyLink::query_by_id(id, get_conn(pool)?.deref()).map_err(|_| "Error inserting tiny_link")
}

pub fn delete_tiny_link(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Ok(())
}

#[cfg(test)]
mod test_orm {
    use super::*;

    use crate::model::user::{NewUser, User};
    use crate::schema::{users, users::dsl::*};
    use diesel::prelude::*;

    #[test]
    fn test_simple() {
        let pool = init_pool("postgres://postgres:postgres@localhost/rshort_link")
            .expect("Failed to create pool");
        let conn: &PgConnection = &pool.get().unwrap();
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let user = NewUser {
            id: &uuid,
            name: "name",
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(conn)
            .expect("exec err?");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<User>(conn)
            .expect("load user error");
        println!("items = {:?}", items);
        let num_del = diesel::delete(users.filter(id.eq(&uuid)))
            .execute(conn)
            .expect("delete user err");
        println!("num_del = {:?}", num_del);
    }
}
