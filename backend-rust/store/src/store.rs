use diesel::prelude::*;

use crate::config::Config;

pub struct Store {
    pub conn: PgConnection,
}

impl Default for Store {
    fn default() -> Self {
        let config = Config::default();
        let conn = PgConnection::establish(&config.db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", config.db_url));
        Store { conn }
    }
}
