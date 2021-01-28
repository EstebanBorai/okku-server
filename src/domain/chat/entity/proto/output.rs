use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::chat::{InputProtoMessageDTO, Message};
use crate::domain::user::User;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "inner")]
pub enum Parcel {
    #[serde(rename = "error")]
    Error(Error),
    #[serde(rename = "alive")]
    Poll,
    #[serde(rename = "joined")]
    Joined(Joined),
    #[serde(rename = "user-joined")]
    UserJoined(UserJoined),
    #[serde(rename = "user-left")]
    UserLeft(UserLeft),
    #[serde(rename = "posted")]
    ForeignMessage(Uuid, InputProtoMessageDTO),
    #[serde(rename = "message")]
    LocalMessage(Message),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Output {
    pub parcel: Parcel,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "code")]
pub enum Error {
    #[serde(rename = "name-taken")]
    NameTaken,
    #[serde(rename = "invalid-name")]
    InvalidName,
    #[serde(rename = "not-joined")]
    NotJoined,
    #[serde(rename = "invalid-message-body")]
    InvalidMessageBody,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Joined {
    pub user: User,
    pub others: Vec<User>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserJoined {
    pub user: User,
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLeft {
    pub user_id: Uuid,
}

impl User {
    pub fn new(id: Uuid, name: &str) -> Self {
        User {
            id,
            name: String::from(name),
        }
    }
}

impl Joined {
    pub fn new(user: User, others: Vec<User>, messages: Vec<Message>) -> Self {
        Joined {
            user,
            others,
            messages,
        }
    }
}

impl UserJoined {
    pub fn new(user: User) -> Self {
        UserJoined { user }
    }
}

impl UserLeft {
    pub fn new(user_id: Uuid) -> Self {
        UserLeft { user_id }
    }
}
