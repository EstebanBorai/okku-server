use warp::Filter;

use crate::application::service::Services;

pub fn with_service(
    service: Services,
) -> impl Filter<Extract = (Services,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}
