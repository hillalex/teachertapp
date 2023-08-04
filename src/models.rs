use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::schools)]
pub struct CreateSchool {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::schools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct School {
    pub id: i32,
    pub name: String,
}
