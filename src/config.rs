use anyhow::Result;
use tokio::fs;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_server: String,
    pub app_port: String,
    pub db_server: String,
    pub db_user: String,
    pub db_name: String, 
    pub db_port: String,
    pub db_password: String,
    pub db_max_connections: u32,
}

impl Config {
    pub async fn new(file_path: &str) -> Result<Config>  {
        let contents = fs::read_to_string(file_path).await?;
        let config: Config = serde_json::from_str(contents.as_str())?;

        Ok(config)
    }
}