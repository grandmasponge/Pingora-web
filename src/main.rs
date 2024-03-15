use std::sync::Arc;

use pingora::{server::{self, configuration::Opt}, services::listening::Service};
use pingore_http_core::intores::IntoResponse;
use pingore_http_core::Response;

//modules
mod async_core;
mod pingore_http_core;


const BASE_TCP_PORT: &str = "localhost:3000";

fn main() {
    // define the server options
    let server_options = Opt {
        upgrade: false,
        daemon: false,
        nocapture: false,
        test: false,
        conf: None,
    };

    
    //creating the pingora service holder aka the pingora server
    let mut pingora_server =
    server::Server::new(Some(server_options)).expect("failed creating pingora server");

    let mut router = pingore_http_core::router::AppRouter::new();
    let call = async_core::asyncfn::CallbackHolder::new(callback);
    let mut  route = pingore_http_core::router::route::new("/".to_string());
    route.get(call);
    router.add_route(route);


    let app = pingore_http_core::app::HttpServer {
        router
    };

    // add the app to the server

   let app = Arc::new(app);

   let mut service = Service::new("http_server".to_string(), app);
   service.add_tcp(BASE_TCP_PORT);

    pingora_server.add_service(service);

   // run forever a bit hopefull but whatever.
    pingora_server.run_forever();
}

async fn callback() -> Response {
    "Hello, World!".to_string()
    .into_response()
}
