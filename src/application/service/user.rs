use std::sync::Arc;

use crate::domain::user;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::profile::Repository as ProfileRepository;
use crate::infrastructure::repository::user::Repository;

use super::ProfileService;

pub type UserService = user::UserService<Repository, ProfileRepository>;

pub fn make_user_service(
    db_pool: &'static DbPool,
    profile_service: Arc<ProfileService>,
) -> UserService {
    UserService::new(
        Repository::new(db_pool, ProfileRepository::new(db_pool)),
        profile_service,
    )
}
