use crate::{
    schema::{self, user},
    store::Store,
};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Store {
    // similar to the service layer
    pub fn create_user(
        &mut self,
        username: String,
        password: String,
    ) -> Result<User, diesel::result::Error> {
        let hashed = hash(password, DEFAULT_COST).unwrap();
        let id = Uuid::new_v4().to_string();

        let new_user = User {
            id,
            username,
            password: hashed,
        };

        let created_user = diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(created_user)
    }

    pub fn get_user(&mut self, username: String) -> Result<User, diesel::result::Error> {
        let db_user = user::table
            .filter(user::username.eq(username))
            .first::<User>(&mut self.conn)?;

        Ok(db_user)
    }
}
