use std::sync::Arc;

use crate::domain::avatar;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::avatar::Repository;
use crate::infrastructure::repository::file::Repository as FileRepository;

use super::file::FileService;

pub type AvatarService = avatar::AvatarService<Repository, FileRepository>;

pub fn make_avatar_service(
    db_pool: &'static DbPool,
    file_service: Arc<FileService>,
) -> AvatarService {
    AvatarService::new(Repository::new(db_pool), file_service)
}
