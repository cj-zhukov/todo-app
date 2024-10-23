use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utils::constants::test::TABLE_NAME;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub fn table_name() -> String {
        TABLE_NAME.to_string()
    }
}