use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::User;
use crate::error::Result;

use super::Profile;

#[async_trait]
pub trait ProfileRepository {
    async fn create(&self, user: &User, first_name: &str, email: &str) -> Result<Profile>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Profile>;
    // async fn set_avatar(&self, id: &Uuid, avatar: &Avatar) -> Result<Profile>;
    // async fn remove_avatar(&self, id: &Uuid) -> Result<Profile>;
    // async fn set_bio(&self, id: &Uuid, bio: &str) -> Result<Profile>;
    // async fn set_birthday(&self, id: &Uuid, birthday: Option<DateTime<Utc>>) -> Result<Profile>;
    // async fn set_names(
    //     &self,
    //     id: &Uuid,
    //     first_name: &str,
    //     surname: Option<&str>,
    // ) -> Result<Profile>;
}
