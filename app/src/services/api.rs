use gloo_net::http::{Request, Response};

use request_domain::login::{LoginRequest, Token};
use request_domain::player::Player;

#[derive(Copy, Clone)]
pub struct Api {
    // token: String,
    url: &'static str,
}

impl Api {
    pub const fn new(url: &'static str) -> Api {
        Self { url }
    }
    pub async fn login(&self, login_request: LoginRequest) -> Result<Token> {
        let url = format!("{}/user/login", self.url);
        let response = Request::post(&url).json(&login_request)?.send().await?;
        into_json(response).await
    }

    pub async fn register(&self, login_request: LoginRequest) -> Result<Token> {
        let url = format!("{}/user/register", self.url);
        let response = Request::post(&url).json(&login_request)?.send().await?;
        into_json(response).await
    }

    pub async fn get_players(&self) -> Vec<Player> {
        let url = format!("{}/player/search", self.url);
        let response = Request::post(&url)
            .header("X-Auth-Token", "")
            .send().await.unwrap();
        response.json().await.unwrap()
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn into_json<T>(response: Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(Box::from("failed".to_string()))
    }
}
