use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::secret::Secret;

#[derive(FromRow)]
pub struct SecretDTO {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}

impl Into<Secret> for SecretDTO {
    fn into(self) -> Secret {
        Secret {
            id: self.id,
            hash: self.hash,
            user_id: self.user_id,
        }
    }
}
