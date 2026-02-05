use crate::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn sign_up() {}

    pub fn sign_in() {}
}
