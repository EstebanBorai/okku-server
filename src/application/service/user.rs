use std::sync::Arc;

use crate::domain::secret;
use crate::domain::user;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::profile::Repository as ProfileRepository;
use crate::infrastructure::repository::secret::Repository as SecretRepository;
use crate::infrastructure::repository::user::Repository;

use super::ProfileService;

pub type SecretService = secret::SecretService<SecretRepository>;
pub type UserService = user::UserService<Repository, ProfileRepository, SecretRepository>;

pub fn make_user_service(
    db_pool: &'static DbPool,
    profile_service: Arc<ProfileService>,
    secret_service: Arc<SecretService>,
) -> UserService {
    UserService::new(Repository::new(db_pool), profile_service, secret_service)
}
