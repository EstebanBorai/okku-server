use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::avatar::Avatar;
use crate::domain::profile::Profile;
use crate::domain::user::User;

#[derive(Debug, FromRow)]
pub struct ProfileDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub email: String,
    pub avatar_id: Option<Uuid>,
    pub surname: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub bio: Option<String>,
}

impl ProfileDTO {
    pub fn into_profile(dto: &ProfileDTO, user: &User, avatar: Option<Avatar>) -> Profile {
        Profile {
            id: dto.id,
            user: user.clone(),
            first_name: dto.first_name.clone(),
            email: dto.email.clone(),
            avatar,
            surname: dto.surname.clone(),
            birthday: dto.birthday,
            bio: dto.bio.clone(),
            contacts: None,
        }
    }
}
