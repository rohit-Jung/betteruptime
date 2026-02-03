use std::env;

pub const DATABASE_URL: &str = "DATABASE_URL";

pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenvy::dotenv().unwrap_or_else(|_| panic!("Faied to load env variables from .env"));
        let db_url = env::var(DATABASE_URL).unwrap_or_else(|_| panic!("DATABASE_URL not provided"));
        Config { db_url }
    }
}
