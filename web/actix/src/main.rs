use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
