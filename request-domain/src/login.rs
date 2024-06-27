use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq )]
pub struct Token {
    pub token: String
}

impl Default for Token {
    fn default() -> Self {
        Token{
            token: "".to_string(),
        }
    }
}