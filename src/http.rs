use reqwest::{Client, ClientBuilder, header::HeaderMap, RequestBuilder};
use crate::types::{SendMessage, ULID, Message, Token, ServerMembers};

#[derive(Debug, Clone)]
pub struct HTTPClient {
    pub token: Token,
    pub client: Client,
    pub base_url: String
}

impl HTTPClient {
    pub fn new(token: Token, base_url: String) -> Self {
        let (header_key, header_value) = token.to_header();

        let mut headers = HeaderMap::new();
        headers.insert(header_key, header_value.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        HTTPClient { token, client, base_url }
    }

    fn post<T: Into<String>>(&self, route: T) -> RequestBuilder {
        self.client.post(format!("https://{}{}", self.base_url, route.into()))
    }

    fn get<T: Into<String>>(&self, route: T) -> RequestBuilder {
        self.client.get(format!("https://{}{}", self.base_url, route.into()))
    }

    pub async fn send_message(&self, channel_id: ULID, message: SendMessage) -> Message {
        self.post(format!("/channels/{channel_id}/messages"))
            .json(&message)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_server_members(&self, server_id: ULID) -> ServerMembers {
        self.get(format!("/servers/{server_id}/members"))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
