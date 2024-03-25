use dotenv;
use std::env;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
}

impl Settings {
    pub fn load_from_env() -> Result<Self, dotenv::Error> {
        dotenv::dotenv().ok();
        let database = DatabaseSettings {
            db_name: env::var("POSTGRES_DB").unwrap(),
            db_host: env::var("POSTGRES_HOST").unwrap(),
            db_port: env::var("POSTGRES_PORT")
                .unwrap()
                .parse::<u16>()
                .expect("Invalid port number"),
            db_user: env::var("POSTGRES_USER").unwrap(),
            db_pass: env::var("POSTGRES_PASSWORD").unwrap(),
        };

        let app_port: u16 = env::var("APP_PORT")
            .unwrap()
            .parse::<u16>()
            .expect("Invalid port number");

        Ok(Settings { database, app_port })
    }
}