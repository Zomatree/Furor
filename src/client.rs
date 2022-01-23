use crate::{
    lib::{
        client::Client,
        event_handler::EventHandler,
        state::State,
        http::HTTPClient,
        types::{
            message::Message,
            channel::Channel
        }
    },
    println
};

use once_cell::sync::OnceCell;
use std::sync::{Mutex, Arc};

pub static CLIENT: OnceCell<Arc<Mutex<Client>>> = OnceCell::new();

struct Events;

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn on_authenticate(&self, state: &mut State, http: &HTTPClient) {
    }

    async fn on_ready(&self, state: &mut State, http: &HTTPClient) {
    }

    async fn on_message(&self, state: &mut State, http: &HTTPClient, message: Message) {
    }

    async fn on_message_update(&self, state: &mut State, http: &HTTPClient, message: Message) {
    }

    async fn on_message_delete(&self, state: &mut State, http: &HTTPClient, message: Message) {
    }

    async fn on_channel_create(&self, state: &mut State, http: &HTTPClient, channel: Channel) {
    }
}

pub async fn connect(token: String, api_url: String) {
    let client = Client::new(api_url, Box::new(Events), token).await;
    CLIENT.set(Arc::new(Mutex::new(client))).unwrap();
}
