use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Secret {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}
