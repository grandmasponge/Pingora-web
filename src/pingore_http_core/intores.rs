use bytes::Bytes;


pub type Response = http::Response<Bytes>;

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(&self) -> Response {
        self.clone()
    }
}

impl IntoResponse for String {
    fn into_response(&self) -> Response {
        http::Response::builder()
            .status(200)
            .header("Content-Type", "text/plain")
            .header("Content-Length", self.len())
            .body(Bytes::from(self.to_string()))
            .unwrap()
    }
}