use anyhow::Result;

use todo_app::{
    utils::constants::{test, DB_NAME_SECRET, PASSWORD_SECRET, DB_USER_SECRET}, 
    Application, 
    DB 
};

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::build(test::DB_ADDRESS, &DB_USER_SECRET, &PASSWORD_SECRET, &DB_NAME_SECRET, 10).await?;
    db.run_migrations().await?;
    let app = Application::build(test::APP_ADDRESS, db).await?;
    app.run().await?;   

    Ok(())
}
