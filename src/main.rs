use anyhow::Result;

use todo_app::{Application, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new("config.json").await?;
    let app = Application::build(config).await?;
    app.run().await?;   

    Ok(())
}
