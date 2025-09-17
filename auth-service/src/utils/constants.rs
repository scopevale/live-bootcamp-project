use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_dburl();
}

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).unwrap_or_else(|_| {
        // For tests, provide a default test secret
        if cfg!(test) {
            "test_secret_key_for_unit_tests_only".to_string()
        } else {
            panic!("JWT_SECRET must be set.");
        }
    });
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_dburl() -> String {
    dotenv().ok(); // Load environment variables
    let dburl = std_env::var(env::DATABASE_URL_ENV_VAR).unwrap_or_else(|_| {
        panic!("DATABASE_URL must be set.");
    });
    if dburl.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    dburl
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
