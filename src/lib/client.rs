use crate::{
    lib::{
        http::HTTPClient,
        ws::WSClient,
        event_handler::EventHandler,
        state::State
    },
    println
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Client {
    pub ws: Arc<Mutex<WSClient>>,
    pub state: State,
}

impl Client {
    pub async fn new(api_url: String, event_handler: Box<dyn EventHandler + Sync + Send + 'static>, token: String) -> Client {
        let http = HTTPClient::new(api_url, token.clone()).await;
        let state = State::new();
        let ws = WSClient::new(http.config.ws.clone(), event_handler, http, token).await;

        Client { ws: Arc::new(Mutex::new(ws)), state }
    }

    pub async fn start(&self) -> State {
        println!("3");
        self.ws.lock().unwrap().ws_loop(self.state.clone()).await
    }
}
