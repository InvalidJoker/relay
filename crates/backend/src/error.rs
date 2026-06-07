#![allow(dead_code)]
use axum::http::StatusCode;

pub(crate) type HttpError = (StatusCode, &'static str);

pub(crate) const PAYLOAD_INCOMPLETE: HttpError = (
    StatusCode::BAD_REQUEST,
    "Payload error: Payload is incomplete",
);
pub(crate) const PAYLOAD_INVALID: HttpError =
    (StatusCode::BAD_REQUEST, "Payload error: Payload is invalid");
