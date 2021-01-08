use std::sync::Arc;

use crate::domain::profile;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::profile::Repository;
use crate::infrastructure::repository::user::Repository as UserRepository;

use super::UserService;

pub type ProfileService = profile::ProfileService<Repository, UserRepository>;

pub fn make_profile_service(
    db_pool: &'static DbPool,
    user_service: Arc<UserService>,
) -> ProfileService {
    ProfileService::new(Repository::new(db_pool), user_service)
}
