use std::{sync::Arc, time::Duration};
use futures::{StreamExt, SinkExt};
use async_std::{sync::Mutex, task::sleep};
use wasm_bindgen_futures::spawn_local;

use crate::{
    lib::{
        types::{
            ws::{ReceiveMessage, SendMessage, ChannelUpdateClear},
            channel::Channel
        },
        event_handler::EventHandler,
        state::State,
        http::HTTPClient,
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

        let heartbeat_ws = self.ws.clone();

        spawn_local(async move {
            loop {
                let payload = SendMessage::Ping { data: 0 };
                heartbeat_ws.lock().await.send(WsMessage::Text(serde_json::to_string(&payload).unwrap())).await.unwrap();
                sleep(Duration::from_secs(30)).await;
            }
        });

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
            ReceiveMessage::Pong { .. } => { },
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
            ReceiveMessage::MessageUpdate { id, channel, data } => {
                let mut found_message = None;

                if let Some(channel) = state.channel_messages.get_mut(&channel) {
                    if let Some(message) = channel.iter_mut().find(|message| message.id == id) {
                        message.edited = Some(data.edited.date.clone());

                        if let Some(new_content) = data.content {
                            message.content = new_content
                        }

                        if let Some(new_embeds) = data.embeds {
                            message.embeds = new_embeds
                        }

                        found_message = Some(message.clone())
                    }
                }

                if let Some(message) = found_message {
                    self.event_handler.on_message_update(state, &self.http, message).await
                }
            },
            ReceiveMessage::MessageDelete { id, channel } => {
                if let Some(channel) = state.channel_messages.get_mut(&channel) {
                    if let Some((i, _)) = channel.iter().enumerate().find(|(_, message)| message.id == id) {
                        let message = channel.remove(i).unwrap();
                        self.event_handler.on_message_delete(state, &self.http, message).await
                    }
                }
            },
            ReceiveMessage::ChannelCreate { channel } => {
                state.channels.insert(channel.id(), channel.clone());
                self.event_handler.on_channel_create(state, &self.http, channel).await
            },
            ReceiveMessage::ChannelUpdate { id, data, clear } => {
                if let Some(channel) = state.channels.get_mut(&id) {
                    if let Some(new_description) = data.description {
                        match channel {
                            Channel::SavedMessages { .. } => { },
                            Channel::DirectMessage { .. } => { },
                            Channel::Group { description, .. } => {
                                *description = Some(new_description)
                            },
                            Channel::TextChannel { description, .. } => {
                                *description = Some(new_description)
                            },
                            Channel::VoiceChannel { description, .. } => {
                                *description = Some(new_description)
                            },
                        }
                    }

                    if let Some(new_name) = data.name {
                        match channel {
                            Channel::SavedMessages { .. } => { },
                            Channel::DirectMessage { .. } => { },
                            Channel::Group { name, .. } => {
                                *name = new_name
                            },
                            Channel::TextChannel { name, .. } => {
                                *name = new_name
                            },
                            Channel::VoiceChannel { name, .. } => {
                                *name = new_name
                            },
                        }
                    }

                    if let Some(new_recipients) = data.recipients {
                        match channel {
                            Channel::SavedMessages { .. } => { },
                            Channel::DirectMessage { .. } => { },
                            Channel::Group { recipients, .. } => {
                                *recipients = new_recipients
                            },
                            Channel::TextChannel { .. } => { },
                            Channel::VoiceChannel { .. } => { },
                        }
                    }

                    if let Some(clear) = clear {
                        match clear {
                            ChannelUpdateClear::Icon => {
                                match channel {
                                    Channel::SavedMessages { .. } => { },
                                    Channel::DirectMessage { .. } => { },
                                    Channel::Group { icon, .. } => {
                                        *icon = None
                                    },
                                    Channel::TextChannel { icon, .. } => {
                                        *icon = None
                                    },
                                    Channel::VoiceChannel { icon, .. } => {
                                        *icon = None
                                    },
                                }
                            },
                            ChannelUpdateClear::Description => {
                                match channel {
                                    Channel::SavedMessages { .. } => { },
                                    Channel::DirectMessage { .. } => { },
                                    Channel::Group { description, .. } => {
                                        *description = None
                                    },
                                    Channel::TextChannel { description, .. } => {
                                        *description = None
                                    },
                                    Channel::VoiceChannel { description, .. } => {
                                        *description = None
                                    },
                                }
                            },
                        }
                    }
                }
            },
            ReceiveMessage::ChannelDelete { id } => { },
            ReceiveMessage::ChannelGroupJoin { id, user } => { },
            ReceiveMessage::ChannelGroupLeave { id, user } => { },
            ReceiveMessage::ChannelStartTyping { id, user } => { },
            ReceiveMessage::ChannelAck { id, user, message_id } => { },
            ReceiveMessage::ServerUpdate { id, data, clear } => { },
            ReceiveMessage::ServerDelete { id } => { },
            ReceiveMessage::ServerMemberUpdate { id, data, clear } => { },
            ReceiveMessage::ServerMemberJoin { id, user } => { },
            ReceiveMessage::ServerMemberLeave { id, user } => { },
            ReceiveMessage::ServerRoleUpdate { id, role_id, data, clear } => { },
            ReceiveMessage::ServerRoleDelete { id, role_id } => { },
            ReceiveMessage::UserUpdate { id, data, clear } => { },
            ReceiveMessage::UserRelationship { id, user, status } => { },
            ReceiveMessage::ChannelStopTyping { id, user } => { },
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
