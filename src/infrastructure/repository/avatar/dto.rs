use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::avatar::Avatar;
use crate::domain::file::File;

#[derive(Debug, FromRow)]
pub struct AvatarDTO {
    pub id: Uuid,
    pub file_id: Uuid,
}

impl AvatarDTO {
    pub fn as_avatar(dto: &AvatarDTO, file: File) -> Avatar {
        Avatar { id: dto.id, file }
    }
}
