use crate::{
    lib::{
        client::Client,
        event_handler::EventHandler,
        state::State,
        http::HTTPClient,
        types::{
            message::Message
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
        println!("authenticated {:?} {:?}", state, http);
    }

    async fn on_ready(&self, state: &mut State, http: &HTTPClient) {
        println!("ready {:?} {:?}", state, http);
    }

    async fn on_message(&self, state: &mut State, http: &HTTPClient, message: Message) {
        println!("{}: {}", state.get_user(&message.author).unwrap().username, message.content);
    }
}

pub async fn connect(token: String, api_url: String) {
    let client = Client::new(api_url, Box::new(Events), token).await;
    CLIENT.set(Arc::new(Mutex::new(client))).unwrap();
}
