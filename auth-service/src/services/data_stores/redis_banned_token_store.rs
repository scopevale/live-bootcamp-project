use std::sync::Arc;

use color_eyre::eyre::{Result, WrapErr};
use redis::{Commands, Connection};
use tokio::sync::RwLock;
use tracing::instrument;

use crate::{
    services::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    #[instrument(name = "new_redis_banned_token_store", skip(conn))]
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

impl std::fmt::Debug for RedisBannedTokenStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisBannedTokenStore")
            .field("conn", &"<redis connection>")
            .finish()
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    #[instrument(name = "ban_token", skip(self, token), fields(token = %token))]
    async fn ban_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(token.as_str());

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("failed to cast TOKEN_TTL_SECONDS to u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(&token_key, value, ttl)
            .wrap_err("failed to set banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    #[instrument(name = "is_token_banned", skip(self, token), fields(token = %token))]
    async fn is_token_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token);

        let is_banned: bool = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .wrap_err("failed to check if token is banned in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

#[instrument(name = "get_key", skip(token), fields(token = %token))]
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
