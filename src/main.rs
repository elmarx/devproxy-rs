use actix_web::{
    client, http::header::HOST, middleware, server, App, AsyncResponder, Body, Error, HttpMessage,
    HttpRequest, HttpResponse,
};
use futures::{Future, Stream};

/// streaming client request to a streaming server response
fn streaming(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let path = req.match_info().get("path").unwrap();
    let host = req
        .headers()
        .get(HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap(); // no host header can not happen in HTTP 1.1, so… unwrap

    // send client request
    client::ClientRequest::get(format!("http://{}/{}", host, path))
        .finish()
        .unwrap()
        .send() // <- connect to host and send request
        .map_err(Error::from) // <- convert SendRequestError to an Error
        .and_then(|resp| {
            // <- we received client response
            Ok(HttpResponse::Ok()
                // read one chunk from client response and send this chunk to a server response
                // .from_err() converts PayloadError to an Error
                .body(Body::Streaming(Box::new(resp.payload().from_err()))))
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("http-proxy");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/{path:.*}", |r| r.f(streaming))
    })
    .workers(1)
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
