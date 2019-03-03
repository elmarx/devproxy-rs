use actix_web::http;
use actix_web::{
    http::header::LOCATION, server, App, Body, HttpMessage, HttpRequest, HttpResponse,
};
use futures::stream::Stream;

fn main() {
    let sys = actix::System::new("devproxy-sample");

    let app_factory = || {
        App::new()
            .route("/", http::Method::GET, |_: HttpRequest| {
                HttpResponse::Ok().body("OK")
            })
            .route("/redirect", http::Method::GET, |_: HttpRequest| {
                HttpResponse::Found().header(LOCATION, "/").finish()
            })
            .route("/header", http::Method::GET, |_: HttpRequest| {
                HttpResponse::Ok()
                    .header("upstream", "42")
                    .body("Sent response header 'upstream: 42'")
            })
            .route("/echo", http::Method::GET, |r: HttpRequest| {
                let mut response = HttpResponse::Ok();

                let body = format!("{:?}", r.headers());

                response.body(body)
            })
            .route("/echo", http::Method::POST, |r: HttpRequest| {
                HttpResponse::Ok().body(Body::Streaming(Box::new(r.payload().from_err())))
            })
            .route(
                "/slow",
                http::Method::GET,
                |_r: HttpRequest| -> HttpResponse {
                    // TODO: implement this: stream 5 single lines every 500ms
                    HttpResponse::NotImplemented()
                        .body("implement this: stream 5 single lines every 500ms")
                },
            )
    };

    server::new(app_factory).bind("[::]:8180").unwrap().start();
    server::new(app_factory).bind("[::]:8280").unwrap().start();

    let _ = sys.run();
}
