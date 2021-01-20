use uuid::Uuid;

pub struct Secret {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}
