use crate::model::User;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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