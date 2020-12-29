use crate::domain::user;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::user::Repository;

pub type UserService = user::UserService<Repository>;

pub fn make_user_service(db_pool: &'static DbPool) -> UserService {
    UserService::new(Repository::new(db_pool))
}
