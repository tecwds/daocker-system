use config::{Config, File};
use lazy_static::lazy_static;
use lombok::{Getter, Setter};
use serde::Deserialize;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Debug, Deserialize, Setter, Getter)]
pub struct Settings {
    server: ServerSettings,
    datasource: DataSourceSettings,
    token: TokenSettings,
}

#[derive(Debug, Deserialize, Setter, Getter)]
pub struct ServerSettings {
    addr: String,
    port: u32,
}

#[derive(Debug, Deserialize, Setter, Getter)]
pub struct DataSourceSettings {
    url: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Setter, Getter)]
pub struct TokenSettings {
    secret_key: String,
    expire: i64,
}

impl Settings {
    pub fn load_settings() -> Result<Settings, Box<dyn std::error::Error>> {
        let settings: Settings = Config::builder()
            .add_source(File::with_name("Daocker.toml"))
            .build()?
            .try_deserialize()?;
        Ok(settings)
    }
}

// #[allow(dead_code)]
fn get_datasource() -> Result<Pool<MySql>, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap().block_on(async {
        MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "mysql://{}:{}@{}",
            SETTINGS.get_datasource().get_username(),
            SETTINGS.get_datasource().get_password(),
            SETTINGS.get_datasource().get_url()
        ))
        .await.unwrap()
    });

    Ok(rt)
}

pub async fn get_pool() -> Result<Pool<MySql>, Box<dyn std::error::Error>> {
    let res = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "mysql://{}:{}@{}",
            SETTINGS.get_datasource().get_username(),
            SETTINGS.get_datasource().get_password(),
            SETTINGS.get_datasource().get_url()
        ))
        .await?;

    Ok(res)
}

#[derive(Getter)]
pub struct SPool {
    pool: Pool<MySql>,
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::load_settings().unwrap();
    pub static ref POOL: SPool = SPool { pool: get_datasource().unwrap() };
}
