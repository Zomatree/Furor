use reqwest::{Client, ClientBuilder, header::HeaderMap, RequestBuilder};
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HTTPClient {
    pub token: types::Token,
    pub user_id: types::ULID,
    pub client: Client,
    pub base_url: &'static str,
    pub revolt_config: types::RevoltConfig
}

impl HTTPClient {
    pub fn new(token: types::Token, user_id: types::ULID, base_url: &'static str, revolt_config: types::RevoltConfig) -> Self {
        let (header_key, header_value) = token.to_header();

        let mut headers = HeaderMap::new();
        headers.insert(header_key, header_value.parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { token, user_id, client, base_url, revolt_config }
    }

    fn post<T: Into<String>>(&self, route: T) -> RequestBuilder {
        self.client.post(format!("{}{}", self.base_url, route.into()))
    }

    fn get<T: Into<String>>(&self, route: T) -> RequestBuilder {
        self.client.get(format!("{}{}", self.base_url, route.into()))
    }

    pub async fn send_message(&self, channel_id: &types::ULID, message: types::SendMessage) -> types::Message {
        self.post(format!("/channels/{channel_id}/messages"))
            .json(&message)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_server_members(&self, server_id: &types::ULID) -> types::ServerMembers {
        self.get(format!("/servers/{server_id}/members"))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_message(&self, channel_id: &types::ULID, message_id: &types::ULID) -> types::Message {
        self.get(format!("/channels/{channel_id}/messages/{message_id}"))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

impl PartialEq for HTTPClient {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
