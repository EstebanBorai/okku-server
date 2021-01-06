use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Debug, FromRow)]
pub struct UserDTO {
    pub id: Uuid,
    pub name: String,
}

impl Into<User> for UserDTO {
    fn into(self) -> User {
        User {
            id: self.id,
            name: self.name,
        }
    }
}
