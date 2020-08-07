use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
struct Info {
    name: String,
}

async fn index(req: HttpRequest, path: web::Path<Info>) -> String {
    println!("REQ: {:?}", req);
    // let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    format!("Hello {}!", path.name) // If "/" => missing field `name`
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
