use crate::service::InjectedServices;
use warp::Filter;

pub fn with_service(
    service: InjectedServices,
) -> impl Filter<Extract = (InjectedServices,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}
