use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    domain::AppState,
    services::{
        HashmapTwoFACodeStore, HashsetBannedTokenStore, MockEmailClient, PostgresUserStore,
    },
    utils::constants::{prod, DATABASE_URL},
    Application,
};

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    let email_client = Arc::new(MockEmailClient);

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> sqlx::PgPool {
    // Create Postgres connection pool
    let pg_pool = sqlx::PgPool::connect(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to migrate the database");
    pg_pool
}
