use std::sync::Arc;
use futures::{StreamExt, SinkExt};
use serde_json::json;
use async_std::{sync::Mutex};
use crate::{
    lib::{
        types::ws::{ReceiveMessage, SendMessage},
        event_handler::EventHandler,
        state::State,
        http::HTTPClient
    },
    println
};
use ws_stream_wasm::{WsStream, WsMeta, WsMessage, WsErr};

pub struct WSClient {
    pub ws: Arc<Mutex<WsStream>>,
    pub token: String,
    event_handler: Box<dyn EventHandler + Sync + Send + 'static>,
    pub http: HTTPClient,
}

impl WSClient {
    pub async fn new(ws_url: String, event_handler: Box<dyn EventHandler + Sync + Send + 'static>, http: HTTPClient, token: String) -> WSClient {
        let (_, ws) = WsMeta::connect(format!("{}?format=json", ws_url), None).await.unwrap();

        WSClient {
            ws: Arc::new(Mutex::new(ws)),
            token,
            event_handler,
            http,
        }
    }

    pub async fn ws_loop(&self, mut state: State) -> State {
        let clone_ws = || self.ws.clone();
        let token = self.token.clone();
        let auth_payload = SendMessage::Authenticate { token };
        clone_ws().lock().await.send(WsMessage::Text(serde_json::to_string(&auth_payload).unwrap())).await.unwrap();

        while let Some(msg) = clone_ws().lock().await.next().await {
            self.handle_message(msg, &mut state).await;
        }

        state
    }

    pub async fn handle_message(&self, msg: WsMessage, state: &mut State) {
        let payload = match msg {
            WsMessage::Text(payload) => payload,
            _ => return
        };

        let event = match serde_json::from_str::<ReceiveMessage>(&payload) {
            Ok(e) => e,
            Err(e) => return println!("{:?}", e)
        };

        match event {
            ReceiveMessage::Authenticated {  } => {
                self.event_handler.on_authenticate(state, &self.http).await
            },
            ReceiveMessage::Error { error } => { },
            ReceiveMessage::Pong { data } => { },
            ReceiveMessage::Ready { users, servers, channels } => {
                for server in servers {
                    state.servers.insert(server.id.clone(), server);
                }

                for user in users {
                    state.users.insert(user.id.clone(), user);
                }

                for channel in channels {
                    state.channels.insert(channel.id(), channel);
                }

                self.event_handler.on_ready(state, &self.http).await
            },
            ReceiveMessage::Message { message } => {
                state.push_message(message.clone());
                self.event_handler.on_message(state, &self.http, message).await;
            },
            ReceiveMessage::MessageUpdate { id, content, edited, embeds } => todo!(),
            ReceiveMessage::MessageDelete { id, channel } => todo!(),
            ReceiveMessage::ChannelCreate { channel } => todo!(),
            ReceiveMessage::ChannelUpdate { id, data, clear } => todo!(),
            ReceiveMessage::ChannelDelete { id } => todo!(),
            ReceiveMessage::ChannelGroupJoin { id, user } => todo!(),
            ReceiveMessage::ChannelGroupLeave { id, user } => todo!(),
            ReceiveMessage::ChannelStartTyping { id, user } => todo!(),
            ReceiveMessage::ChannelAck { id, user, message_id } => todo!(),
            ReceiveMessage::ServerUpdate { id, data, clear } => todo!(),
            ReceiveMessage::ServerDelete { id } => todo!(),
            ReceiveMessage::ServerMemberUpdate { id, data, clear } => todo!(),
            ReceiveMessage::ServerMemberJoin { id, user } => todo!(),
            ReceiveMessage::ServerMemberLeave { id, user } => todo!(),
            ReceiveMessage::ServerRoleUpdate { id, role_id, data, clear } => todo!(),
            ReceiveMessage::ServerRoleDelete { id, role_id } => todo!(),
            ReceiveMessage::UserUpdate { id, data, clear } => todo!(),
            ReceiveMessage::UserRelationship { id, user, status } => todo!(),
        }
    }

    pub async fn handle_error(&self, error: WsErr) {

    }
}

impl std::fmt::Debug for WSClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WSClient")
            .field("token", &self.token)
            .field("http", &self.http)
            .finish()
    }
}
