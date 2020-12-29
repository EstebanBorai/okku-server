use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::server::utils::{Message, Response};

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if let Some(response) = err.find::<Response<Message>>() {
        code = StatusCode::from_u16(response.get_status_code()).unwrap();
        message = response.get_message();
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("Unhandled Rejection");
    }

    let res = Response::message(message).status_code(code);
    let json = warp::reply::json(&res);

    Ok(warp::reply::with_status(json, code))
}
