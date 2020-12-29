use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct SecretDTO {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}
