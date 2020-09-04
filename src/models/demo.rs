use crate::db::establish_connection;
use crate::schema::demos;
use crate::schema::demos::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "demos"]
pub struct Demo {
    pub name: String,
    pub demo_text: String,
    pub favorite_number: i32,
}
#[derive(Queryable, Serialize, AsChangeset, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "demos"]
pub struct Demos {
    pub id: i32,
    pub name: String,
    pub demo_text: String,
    pub favorite_number: i32,
}

impl Demos {
    /// read DAO for Demos
    pub fn read() -> Result<Vec<Self>, Error> {
        let db_conn = establish_connection();
        demos.load(&db_conn)
    }
    /// read by id DAO for Demos
    pub fn read_by_id(demo_id: i32) -> Result<Self, Error> {
        let db_conn = establish_connection();
        demos.filter(demos::id.eq(demo_id)).first(&db_conn)
    }
    /// create DAO for Demos
    pub fn create(new_demo: Demo) -> Result<Self, Error> {
        let db_conn = establish_connection();
        diesel::insert_into(demos)
            .values(new_demo)
            .get_result(&db_conn)
    }
    /// update DAO for Demos
    pub fn update(demo_id: i32, existing_demo: Demos) -> Result<Self, Error> {
        let db_conn = establish_connection();
        diesel::update(demos.find(demo_id))
            .filter(demos::id.eq(id))
            .set(existing_demo)
            .get_result(&db_conn)
    }
    /// delete DAO for Demo
    pub fn delete(demo_id: i32) -> Result<usize, Error> {
        let db_conn = establish_connection();
        diesel::delete(demos.filter(demos::id.eq(demo_id))).execute(&db_conn)
    }
}
