use serde::Serialize;
use warp::http::StatusCode;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::profile::Profile;
use crate::domain::user::User;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct MeResponse {
    user: User,
    profile: Profile,
}

pub async fn me(
    claims: Claims,
    services: Services,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    match tokio::try_join!(
        services.user_service.find_by_id(&claims.user_id),
        services.user_service.fetch_profile(&claims.user_id)
    ) {
        Ok((user, profile)) => {
            Ok(Response::new(MeResponse { user, profile }).status_code(StatusCode::OK))
        }
        Err(e) => Err(Response::reject_with(e, StatusCode::FORBIDDEN)),
    }
}
