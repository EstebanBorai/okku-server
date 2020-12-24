use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::user::User;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(id: Uuid, user: User, body: &str, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            user,
            body: String::from(body),
            created_at,
        }
    }
}
