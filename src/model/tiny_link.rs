use crate::schema::{
    tiny_link,
    tiny_link::dsl::{id, origin, tiny, tiny_link as all_tiny_link},
};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Queryable)]
pub struct TinyLink {
    pub id: String,
    pub tiny: String,
    pub origin: String,
    pub user_id: String,
}

#[derive(Insertable)]
#[table_name = "tiny_link"]
pub struct NewTinyLink<'a> {
    pub id: &'a str,
    pub tiny: &'a str,
    pub origin: &'a str,
}
