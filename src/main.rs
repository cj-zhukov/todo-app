use anyhow::Result;

use todo_app::{
    Application, 
    config::Config,
    utils::constants::test,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new("config.json").await?;
    let app = Application::build(config, test::APP_ADDRESS).await?;
    app.run().await?;   

    Ok(())
}
