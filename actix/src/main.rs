use actix::{Actor, StreamHandler};
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use env_logger::Env;
use std::sync::Mutex;

/// Define shared state
struct AppState {
    name: Mutex<String>, // <- Mutex is necessary to mutate safely across threads
}

/// Define http actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn index(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    // let name = req.match_info().get("name").unwrap_or("World");
    let mut name = data.name.lock().unwrap(); // <- get MutexGuard

    if let Some(req_name) = req.match_info().get("name") {
        *name = String::from(req_name);
    }

    format!("Hello {}!", &name)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let state = web::Data::new(AppState {
        name: Mutex::new(String::from("World")),
    });

    let server_url = String::from("0.0.0.0");
    let server_port = String::from("8080");

    // if dotenv::dotenv().ok().is_some() {
    // server_url = dotenv::var("SERVER_URL").unwrap();
    // server_port = dotenv::var("SERVER_PORT").unwrap();
    // }

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .service(hello)
            .route("/ws/", web::get().to(ws))
            .route("/{name}", web::get().to(index))
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", server_url, server_port))?
    .run()
    .await
}
