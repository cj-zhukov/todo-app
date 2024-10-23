use lazy_static::lazy_static;

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:8080";
    pub const DB_ADDRESS: &str = "127.0.0.1:5432";
    pub const DB_MAX_CONNECTIONS: u32 = 10;
}

lazy_static! {
    pub static ref DB_NAME_SECRET: String = creds::set_db_name();
    pub static ref DB_USER_SECRET: String = creds::set_user_name();
    pub static ref PASSWORD_SECRET: String = creds::set_db_pwd();
}

mod creds {
    use dotenvy::dotenv;
    use std::env as std_env;

    use super::env;
    
    pub fn set_db_name() -> String {
        dotenv().ok();
        let secret = std_env::var(env::DB_NAME_ENV_VAR)
            .expect("DB_NAME_SECRET must be set.");
        if secret.is_empty() {
            panic!("DB_NAME_SECRET must not be empty.");
        }
        secret
    }

    pub fn set_user_name() -> String {
        dotenv().ok();
        let secret = std_env::var(env::DB_USER_ENV_VAR)
            .expect("DB_USER_NAME_SECRET must be set.");
        if secret.is_empty() {
            panic!("DB_USER_NAME_SECRET must not be empty.");
        }
        secret
    }

    pub fn set_db_pwd() -> String {
        dotenv().ok();
        let secret = std_env::var(env::PASSWORD_ENV_VAR)
            .expect("PASSWORD_SECRET must be set.");
        if secret.is_empty() {
            panic!("PASSWORD_SECRET must not be empty.");
        }
        secret
    }
}

pub mod env {
    pub const DB_NAME_ENV_VAR: &str = "DB_NAME";
    pub const DB_USER_ENV_VAR: &str = "DB_USER";
    pub const PASSWORD_ENV_VAR: &str = "PASSWORD";
}