use reqwest::{Client, ClientBuilder, header::HeaderMap, RequestBuilder, Method, Response, Error as ReqwestError, multipart::{Form, Part}};
use std::{
    collections::HashMap,
    rc::Rc,
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
    pub revolt_config: Rc<types::RevoltConfig>,
    pub ratelimits: Rc<Mutex<HashMap<String, Ratelimit>>>,
    pub default_headers: HeaderMap
}

impl HTTPClient {
    pub fn new(token: types::Token, user_id: types::ULID, base_url: &'static str, revolt_config: types::RevoltConfig) -> Self {
        let (header_key, header_value) = token.to_header();

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header_key, header_value.parse().unwrap());

        let client = ClientBuilder::new()
            .build()
            .unwrap();

        Self { token, user_id, client, base_url, revolt_config: Rc::new(revolt_config), ratelimits: Rc::new(Mutex::new(HashMap::new())), default_headers }
    }

    #[inline]
    fn build<T: AsRef<str>>(&self, method: Method, route: T) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.base_url, route.as_ref()))
            .headers(self.default_headers.clone())

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
    async fn send_handle_rl(&self, builder: RequestBuilder) -> Result<Response, ReqwestError> {
        let resp = builder
            .try_clone()
            .unwrap()
            .send()
            .await
            .unwrap();

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

                self.send_handle_rl(builder).await
            },
            _ => Ok(resp)
        }
    }

    async fn send(&self, builder: RequestBuilder) -> Result<Response, ReqwestError> {
        builder
            .send()
            .await
    }

    pub async fn send_message(&self, channel_id: &types::ULID, message: types::SendMessage) -> types::Message {
        self.send_handle_rl(
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
        self.send_handle_rl(
            self.get(format!("/servers/{server_id}/members"))
        )
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn fetch_message(&self, channel_id: &types::ULID, message_id: &types::ULID) -> types::Message {
        self.send_handle_rl(
            self.get(format!("/channels/{channel_id}/messages/{message_id}"))
        )
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn delete_message(&self, channel_id: &types::ULID, message_id: &types::ULID) {
        self.send_handle_rl(
            self.delete(format!("/channels/{channel_id}/messages/{message_id}"))
        )
            .await
            .unwrap();
    }

    pub async fn upload_file(&self, tag: &'static str, content: Vec<u8>, filename: String) -> types::AutumnResponse {
        let multipart = reqwest::multipart::Form::new()
            .part("file",
                Part::bytes(content)
                .file_name(filename)
            );

        let request = self.client.request(Method::POST, &format!("{}/{tag}", self.revolt_config.features.autumn.url))
            .multipart(multipart);

        self.send(request)
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }

    pub async fn edit_message(&self, channel_id: &types::ULID, message_id: &types::ULID, message: types::SendMessage) {
        self.send(
            self.patch(format!("/channels/{channel_id}/messages/{message_id}"))
            .json(&message)
        )
            .await
            .unwrap();
    }
}

impl PartialEq for HTTPClient {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
