use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::{
    tiny_link,
    tiny_link::dsl::{id, tiny_link as all_tiny_link,},
};

#[derive(Deserialize, Serialize, Queryable, Debug, PartialEq)]
pub struct TinyLink {
    pub id: String,
    pub tiny: String,
    pub origin: String,
    pub user_id: Option<String>,
}

impl TinyLink {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<TinyLink>> {
        all_tiny_link.order(tiny_link::id.desc()).load::<TinyLink>(conn)
    }

    pub fn insert(link: NewTinyLink, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tiny_link::table)
            .values(&link)
            .execute(conn)
    }
    pub fn query_by_id(target_id: &str, conn: &PgConnection) ->QueryResult<TinyLink> {
        all_tiny_link.filter(id.eq(target_id)).first::<TinyLink>(conn)
    }
}

#[derive(Deserialize, Serialize, Insertable, Debug, Clone, PartialEq)]
#[table_name = "tiny_link"]
pub struct NewTinyLink<'a> {
    pub id: String,
    pub tiny: String,
    pub origin: &'a str,
    pub user_id: Option<String>,
}

impl<'a> NewTinyLink<'a> {
    pub fn new(link: &'a str, user_id: &Option<String>) -> NewTinyLink<'a> {
        let tiny_id: String = Uuid::new_v4().to_string();
        NewTinyLink::<'a> {
            id: tiny_id.clone(),
            tiny: tiny_id.clone(),
            origin: link,
            user_id: user_id.clone(),
        }
    }
    pub fn to_tiny_link(&self, user_id: Option<String>) -> TinyLink {
        TinyLink {
            id: self.id.to_string(),
            tiny: self.tiny.to_string(),
            origin: self.origin.to_string(),
            user_id: None,
        }
    }
}

#[cfg(test)]
mod test_new_tiny_link {
    use super::*;

    #[test]
    fn test_new() {
        let input = NewTinyLink { origin: "aaaaa" };
        println!("input = {:?}", input);
        println!("input = {:?}", input.to_tiny_link(Some("user_id_01")));
    }
}
