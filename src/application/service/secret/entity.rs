use uuid::Uuid;

use crate::infrastructure::repository::secret::SecretDTO;

pub struct Secret {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
}

impl From<SecretDTO> for Secret {
    fn from(dto: SecretDTO) -> Self {
        Self {
            id: dto.id,
            hash: dto.hash,
            user_id: dto.user_id,
        }
    }
}
