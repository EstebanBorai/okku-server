use serde::Serialize;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::{Reply, Response};

#[derive(Serialize)]
pub struct HttpResponse<T>
where
    T: std::marker::Sized + std::marker::Send + Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<T>,
}

impl<T> HttpResponse<T>
where
    T: std::marker::Sized + std::marker::Send + Serialize,
{
    pub fn new(message: &str, status_code: StatusCode) -> Self {
        Self {
            message: Some(String::from(message)),
            status_code: status_code.as_u16(),
            payload: None,
        }
    }

    pub fn with_payload(payload: T, status_code: StatusCode) -> Self {
        Self {
            message: None,
            status_code: status_code.as_u16(),
            payload: Some(payload),
        }
    }
}

impl<T> Reply for HttpResponse<T>
where
    T: std::marker::Sized + std::marker::Send + Serialize,
{
    fn into_response(self) -> Response {
        let builder = warp::http::Response::builder().status(
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        );

        builder
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}
