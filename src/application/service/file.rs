use crate::domain::file;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::file::Repository;

pub type FileService = file::FileService<Repository>;

pub fn make_file_service(db_pool: &'static DbPool) -> FileService {
    FileService::new(Repository::new(db_pool))
}
