use crate::clap_app::get_clap_app;
use crate::config::DevproxyConfig;
use actix_web::{
    client, http::header::HOST, middleware, server, App, AsyncResponder, Body, Error, HttpMessage,
    HttpRequest, HttpResponse,
};
use futures::{Future, Stream};
use std::path::PathBuf;

mod clap_app;
mod config;
mod mapper;

/// streaming client request to a streaming server response
fn streaming(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let path = req.match_info().get("path").unwrap();
    let host = req
        .headers()
        .get(HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap(); // no host header can not happen in HTTP 1.1, so… unwrap

    let mut client_request_builder = client::ClientRequest::build();

    client_request_builder
        .no_default_headers() // do not add neither UserAgent nor Host-Header, just pass through the values from downstream
        .method(req.method().clone())
        .uri(format!("http://{}/{}", host, path));

    // attach all headers from the downstream-request to the upstream-request
    req.headers().iter().for_each(|(key, value)| {
        client_request_builder.header(key, value.clone());
    });

    client_request_builder
        .body(Body::Streaming(Box::new(req.payload().from_err())))
        .unwrap()
        .send() // <- connect to host and send request
        .map_err(Error::from) // <- convert SendRequestError to an Error
        .and_then(|resp| {
            let mut response_builder = HttpResponse::build(resp.status());

            // attach upstream-response headers to the downstream-response
            resp.headers().iter().for_each(|(key, value)| {
                response_builder.header(key, value.clone());
            });

            Ok(response_builder.body(Body::Streaming(Box::new(resp.payload().from_err()))))
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let matches = get_clap_app().get_matches();

    let config_file = matches.value_of("config").map(PathBuf::from);

    let config = dbg!(DevproxyConfig::new(config_file));

    let sys = actix::System::new("http-proxy");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/{path:.*}", |r| r.f(streaming))
    })
    .bind(config.addr)
    .unwrap()
    .start();

    println!("Started http server: {}", config.addr);
    let _ = sys.run();
}
