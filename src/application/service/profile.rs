use crate::domain::profile;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::profile::Repository;

pub type ProfileService = profile::ProfileService<Repository>;

pub fn make_profile_service(db_pool: &'static DbPool) -> ProfileService {
    ProfileService::new(Repository::new(db_pool))
}
