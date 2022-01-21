use super::types::{user::User, server::Server, channel::Channel, member::Member, message::Message};
use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug, Clone)]
pub struct State {
    pub users: HashMap<String, User>,
    pub servers: HashMap<String, Server>,
    pub channels: HashMap<String, Channel>,
    pub members: HashMap<String, Member>,
    pub channel_messages: HashMap<String, VecDeque<Message>>
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn push_message(&mut self, message: Message) {
        let deque = self.channel_messages.entry(message.channel.clone()).or_insert_with(VecDeque::new);
        deque.push_front(message);
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }
}
