use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utils::constants::test::TABLE_NAME;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub body: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Todo {
    pub fn table_name() -> String {
        TABLE_NAME.to_string()
    }
}