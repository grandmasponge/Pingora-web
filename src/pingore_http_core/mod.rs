pub mod app;
pub mod router;
pub mod response;
pub mod intores;

pub type Response = http::Response<bytes::Bytes>;