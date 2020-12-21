use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct Secret {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}
