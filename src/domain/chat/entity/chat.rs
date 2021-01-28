use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Message;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chat {
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub participants_ids: Vec<Uuid>,
}

impl Chat {
    pub fn append_message(&mut self, message: Message) {
        self.messages.push(message);
        self.messages.sort_by_key(|msg| msg.created_at);
    }

    pub fn messages_iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }
}
