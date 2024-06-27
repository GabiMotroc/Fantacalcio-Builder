use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use request_domain::login::Token;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
pub struct AuthSession {}

impl AuthSession {
    pub fn save_token(token_str: &str) {
        let (_, set_token, _) = use_local_storage::<Token, JsonCodec>("token");
        set_token(Token {
            token: token_str.to_string(),
        });
    }

    pub fn validate_token(token: &str) -> Result<(), jwt::Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
        let _: BTreeMap<String, i32> = token.verify_with_key(&key)?;

        Ok(())
    }
}
