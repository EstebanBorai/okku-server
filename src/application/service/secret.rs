use crate::domain::secret;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::secret::Repository;

pub type SecretService = secret::SecretService<Repository>;

pub fn make_secret_service(db_pool: &'static DbPool) -> SecretService {
    secret::SecretService::new(Repository::new(db_pool))
}
