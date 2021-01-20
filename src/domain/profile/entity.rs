use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::avatar::Avatar;
use crate::domain::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<Avatar>,
    pub surname: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub contacts: Option<Vec<User>>,
    pub bio: Option<String>,
}
