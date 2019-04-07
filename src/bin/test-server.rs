use actix_web::{
    http::header::LOCATION, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};

fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("devproxy-sample");

    let app_factory = || {
        App::new()
            .wrap(middleware::Logger::default())
            .route(
                "/",
                web::get().to(|_: HttpRequest| HttpResponse::Ok().body("OK")),
            )
            .route(
                "/redirect",
                web::get()
                    .to(|_: HttpRequest| HttpResponse::Found().header(LOCATION, "/").finish()),
            )
            .route(
                "/header",
                web::get().to(|_: HttpRequest| {
                    HttpResponse::Ok()
                        .header("upstream", "42")
                        .body("Sent response header 'upstream: 42'")
                }),
            )
            .route(
                "/echo",
                web::get().to(|r: HttpRequest| {
                    let mut response = HttpResponse::Ok();

                    let body = format!("{:?}", r.headers());

                    response.body(body)
                }),
            )
            .route(
                "/echo",
                web::post().to(|_r: HttpRequest, payload: web::Payload| {
                    HttpResponse::Ok().streaming(payload)
                }),
            )
            .route(
                "/slow",
                web::get().to(|_r: HttpRequest| -> HttpResponse {
                    // TODO: implement this: stream 5 single lines every 500ms
                    HttpResponse::NotImplemented()
                        .body("implement this: stream 5 single lines every 500ms")
                }),
            )
    };

    HttpServer::new(app_factory)
        .bind("[::]:8180")
        .unwrap()
        .start();
    HttpServer::new(app_factory)
        .bind("[::]:8280")
        .unwrap()
        .start();

    sys.run()
}
