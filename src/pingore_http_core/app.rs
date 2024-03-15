use std::sync::Arc;

use async_trait::async_trait;
use pingora::{apps::HttpServerApp, http::ResponseHeader, protocols::{http::ServerSession, Stream}, server::ShutdownWatch};
use log::{debug, error, info, trace};
use crate::async_core::asyncfn::CallbackHolder;

use super::{intores::IntoResponse, response::HttpResponse, router::{route, AppRouter}};

pub struct HttpServer {
    pub router: AppRouter
}

#[async_trait]
impl HttpServerApp for HttpServer {
    async fn process_new_http(
        self: &Arc<Self>,
        mut http: ServerSession,
        shutdown: &ShutdownWatch,) -> Option<Stream> {
            match http.read_request().await {
                Ok(res) => match res {
                    false => {
                        debug!("Failed to read request header");
                        return None;
                    }
                    true => {
                        debug!("Successfully get a new request");
                    }
                },
                Err(e) => {
                    error!("HTTP server fails to read from downstream: {e}");
                    return None;
                }
            };

            trace!("{:?}", http.req_header());
            if *shutdown.borrow() {
                http.set_keepalive(None);
            } else {
                http.set_keepalive(Some(60));
            }

            let response = self.response(&http).await;
            let http_res =  response.into_response();
            let (parts, body) = http_res.into_parts();


            let headers: ResponseHeader = parts.into();
        

            match http.write_response_header(Box::new(headers)).await {
                Ok(()) => {
                    debug!("HTTP response header done.");
                }
                Err(e) => {
                    error!(
                        "HTTP server fails to write to downstream: {e}, {}",
                        http.request_summary()
                    );
                }
            }
            if !body.is_empty() {
                // TODO: check if chunked encoding is needed
                match http.write_response_body(body.into()).await {
                    Ok(_) => debug!("HTTP response written."),
                    
                    Err(e) => error!(
                        "HTTP server fails to write to downstream: {e}, {}",
                        http.request_summary()
                    ),
                }
            }

            match http.finish().await {
                Ok(res) => {
                    res
                },
                Err(e) => {
                    error!("HTTP server fails to finish the request: {e}");
                    None
                }
            }
    }
}  

#[async_trait]
impl HttpResponse for HttpServer {
    async fn response(self: &Self, http: &ServerSession) -> http::Response<bytes::Bytes> {
        let uri = http
        .clone()
        .req_header()
        .uri
        .path()
        .to_string();

        let route = self.router.get_route(uri);
        let a = match route {
            Some(res) => res,
            None => {
                return http::Response::builder()
                .status(404)
                .body(bytes::Bytes::from("Not Found"))
                .unwrap()
            }
        };
        let method = http.req_header().method.clone();
        let callback = match method {
            http::Method::GET => &a.get,
            http::Method::POST => &a.post,
            http::Method::PUT => &a.put,
            http::Method::DELETE => &a.delete,
            _ => &None
            
        };
        let b = match callback {
            Some(res) => res,
            None => {
                return http::Response::builder()
                .status(404)
                .body(bytes::Bytes::from("Not Found"))
                .unwrap()
            }
        };
        let finale = b.callback.create_callback().await;
        finale
    }
}
