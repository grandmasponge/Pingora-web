use bytes::Bytes;
use pingora::protocols::http::ServerSession;
use super::intores::IntoResponse;


#[async_trait::async_trait]
pub trait HttpResponse {
    async fn response(self: &Self, http: &ServerSession) -> http::Response<Bytes>;
}   