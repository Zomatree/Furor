use crate::lib::{
    state::State,
    http::HTTPClient,
    types::{
        message::Message,
        channel::Channel
    }
};

#[async_trait::async_trait]
pub trait EventHandler: 'static {
    async fn on_authenticate(&self, state: &mut State, http: &HTTPClient);
    async fn on_ready(&self, state: &mut State, http: &HTTPClient);
    async fn on_message(&self, state: &mut State, http: &HTTPClient, message: Message);
    async fn on_message_update(&self, state: &mut State, http: &HTTPClient, message: Message);
    async fn on_message_delete(&self, state: &mut State, http: &HTTPClient, message: Message);
    async fn on_channel_create(&self, state: &mut State, http: &HTTPClient, channel: Channel);
}
