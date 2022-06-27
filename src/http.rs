use reqwest::{Client, ClientBuilder, header::HeaderMap, RequestBuilder, Method, Response, Error as ReqwestError};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration
};
use async_std::{
    sync::Mutex,
    task::sleep
};
use async_recursion::async_recursion;

use crate::prelude::*;

#[derive(Debug)]
pub struct Ratelimit {
    pub limit: u16,
    pub remaining: u16,
    pub reset_after: u64
}

#[derive(Debug, Clone)]
pub struct HTTPClient {
    pub token: types::Token,
    pub user_id: types::ULID,
    pub client: Client,
    pub base_url: &'static str,
    pub revolt_config: Arc<types::RevoltConfig>,
    pub ratelimits: Arc<Mutex<HashMap<String, Ratelimit>>>
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

        Self { token, user_id, client, base_url, revolt_config: Arc::new(revolt_config), ratelimits: Arc::new(Mutex::new(HashMap::new())) }
    }

    #[inline]
    fn build<T: AsRef<str>>(&self, method: Method, route: T) -> RequestBuilder {
        self.client.request(method, format!("{}{}", self.base_url, route.as_ref()))
    }

    #[inline]
    fn get<T: AsRef<str>>(&self, route: T) -> RequestBuilder {
        self.build(Method::GET, route)
    }

    #[inline]
    fn post<T: AsRef<str>>(&self, route: T) -> RequestBuilder {
        self.build(Method::POST, route)
    }

    #[inline]
    fn delete<T: AsRef<str>>(&self, route: T) -> RequestBuilder {
        self.build(Method::DELETE, route)
    }

    #[inline]
    fn patch<T: AsRef<str>>(&self, route: T) -> RequestBuilder {
        self.build(Method::PATCH, route)
    }

    #[inline]
    fn put<T: AsRef<str>>(&self, route: T) -> RequestBuilder {
        self.build(Method::PUT, route)
    }

    #[async_recursion(?Send)]
    async fn send(&self, builder: RequestBuilder) -> Result<Response, ReqwestError> {
        let resp = builder
            .try_clone()
            .unwrap()
            .send()
            .await?;

        match resp.status().as_u16() {
            200..=299 => {
                /*
                let headers = resp.headers();

                if let Some(ratelimit_limit) = headers.get("x-ratelimit-limit") &&
                    let Some(ratelimit_bucket) = headers.get("x-ratelimit-bucket") &&
                    let Some(ratelimit_remaining) = headers.get("x-ratelimit-remaining") &&
                    let Some(ratelimit_reset_after) = headers.get("x-ratelimit-reset-after")
                {
                    let ratelimit = Ratelimit {
                        limit: ratelimit_limit.to_str().unwrap().parse().unwrap(),
                        remaining: ratelimit_remaining.to_str().unwrap().parse().unwrap(),
                        reset_after: ratelimit_reset_after.to_str().unwrap().parse().unwrap()
                    };

                    let bucket = ratelimit_bucket.to_str().unwrap().to_string();

                    self.ratelimits.lock().await.insert(bucket, ratelimit);

                };
                */

                Ok(resp)
            },
            429 => {
                let body = resp
                    .json::<types::TooManyRequests>()
                    .await
                    .unwrap();

                sleep(Duration::from_millis(body.retry_after)).await;

                self.send(builder).await
            },
            _ => Ok(resp)
        }
    }

    pub async fn send_message(&self, channel_id: &types::ULID, message: types::SendMessage) -> types::Message {
        self.send(
            self.post(format!("/channels/{channel_id}/messages"))
            .json(&message)
        )
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_server_members(&self, server_id: &types::ULID) -> types::ServerMembers {
        self.send(
            self.get(format!("/servers/{server_id}/members"))
        )
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_message(&self, channel_id: &types::ULID, message_id: &types::ULID) -> types::Message {
        self.send(
            self.get(format!("/channels/{channel_id}/messages/{message_id}"))
        )
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
