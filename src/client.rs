use super::models::Habit;
use reqwest::{Method, Response};

const API_KEY_HEADER: &str = "X-API-Key";

pub struct Client<'a> {
    api_key: &'a str,
    host: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(api_key: &'a str, host: &'a str) -> Client<'a> {
        Client {
            api_key,
            host,
            client: reqwest::Client::new(),
        }
    }

    async fn request(&self, method: Method, path: &str) -> Result<Response, reqwest::Error> {
        self.client
            .request(method, format!("{}{}", self.host, path))
            .header(API_KEY_HEADER, self.api_key)
            .send()
            .await
    }

    pub async fn get_habits(&self) -> Result<Vec<Habit>, reqwest::Error> {
        self.request(Method::GET, "/api/habits")
            .await?
            .json::<Vec<Habit>>()
            .await
    }
}
