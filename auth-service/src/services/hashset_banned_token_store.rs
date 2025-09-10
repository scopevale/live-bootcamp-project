use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn ban_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }

    async fn is_token_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ban_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token_123".to_string();
        
        let result = store.ban_token(token.clone()).await;
        assert_eq!(result, Ok(()));
        
        // Verify the token was actually stored
        assert!(store.banned_tokens.contains(&token));
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        let mut store = HashsetBannedTokenStore::default();
        let banned_token = "banned_token_456".to_string();
        let clean_token = "clean_token_789";
        
        // Ban a token
        store.banned_tokens.insert(banned_token.clone());
        
        // Check banned token
        let result = store.is_token_banned(&banned_token).await;
        assert_eq!(result, Ok(true));
        
        // Check clean token
        let result = store.is_token_banned(clean_token).await;
        assert_eq!(result, Ok(false));
    }

    #[tokio::test]
    async fn test_ban_multiple_tokens() {
        let mut store = HashsetBannedTokenStore::default();
        let tokens = vec![
            "token1".to_string(),
            "token2".to_string(),
            "token3".to_string(),
        ];
        
        // Ban multiple tokens
        for token in &tokens {
            let result = store.ban_token(token.clone()).await;
            assert_eq!(result, Ok(()));
        }
        
        // Verify all tokens are banned
        for token in &tokens {
            let result = store.is_token_banned(token).await;
            assert_eq!(result, Ok(true));
        }
        
        // Verify clean token is not banned
        let clean_token = "clean_token";
        let result = store.is_token_banned(clean_token).await;
        assert_eq!(result, Ok(false));
    }

    #[tokio::test]
    async fn test_ban_duplicate_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "duplicate_token".to_string();
        
        // Ban token twice
        let result1 = store.ban_token(token.clone()).await;
        let result2 = store.ban_token(token.clone()).await;
        
        assert_eq!(result1, Ok(()));
        assert_eq!(result2, Ok(()));
        
        // Should still be banned only once (HashSet behavior)
        assert_eq!(store.banned_tokens.len(), 1);
        let result = store.is_token_banned(&token).await;
        assert_eq!(result, Ok(true));
    }
}