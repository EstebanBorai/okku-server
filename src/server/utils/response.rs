use std::marker::Sized;

use serde::Serialize;
use warp::http::header;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reject::Reject;
use warp::reply::{Reply, Response as WarpResponse};
use warp::Rejection;

use crate::error::Error;

#[derive(Serialize, Debug)]
pub struct Response<T>
where
    T: Sized + std::marker::Send + Serialize,
{
    #[serde(skip_serializing)]
    status_code: u16,
    #[serde(flatten)]
    body: T,
}

impl Reject for Response<Message> {}

#[derive(Clone, Debug, Serialize)]
pub struct Message {
    message: String,
}

impl<T> Response<T>
where
    T: Sized + std::marker::Send + Serialize,
{
    pub fn new(body: T) -> Self {
        Self {
            status_code: 200_u16,
            body,
        }
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code.as_u16();

        self
    }
}

impl Response<Message> {
    pub fn message(message: String) -> Self {
        let body = Message { message };

        Response {
            status_code: 200_u16,
            body,
        }
    }

    pub fn reject_with(e: Error, status_code: StatusCode) -> Rejection {
        let response = Response::message(e.message()).status_code(status_code);

        response.reject()
    }

    pub fn get_message(&self) -> String {
        self.body.message.clone()
    }

    pub fn reject(self) -> Rejection {
        warp::reject::custom(self)
    }
}

impl Response<Vec<u8>> {
    pub fn send_file(bytes: Vec<u8>, content_type: &str) -> WarpResponse {
        let mut response = WarpResponse::new(bytes.into());

        response.headers_mut().insert(
            header::CONTENT_TYPE,
            warp::http::HeaderValue::from_str(content_type).unwrap(),
        );

        *response.status_mut() = StatusCode::OK;

        return response;
    }
}

impl<T> Reply for Response<T>
where
    T: Sized + std::marker::Send + Serialize,
{
    fn into_response(self) -> WarpResponse {
        let builder = warp::http::Response::builder().status(
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        );

        builder
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}

impl<T> Into<WarpResponse> for Response<T>
where
    T: Sized + std::marker::Send + Serialize,
{
    fn into(self) -> WarpResponse {
        let as_json = serde_json::to_string(&self).unwrap();
        let mut response = WarpResponse::new(as_json.into());

        *response.status_mut() = StatusCode::from_u16(self.status_code).unwrap();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            warp::http::HeaderValue::from_static("application/json"),
        );

        response
    }
}
