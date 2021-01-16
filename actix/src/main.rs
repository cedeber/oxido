use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_files as fs;
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use env_logger::Env;
use serde::Deserialize;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define shared state
struct AppState {
    name: Mutex<String>, // <- Mutex is necessary to mutate safely across threads
}

/// Define http actor. Websocket connection is long running connection,
/// it is easier to handle with an actor
struct MyWebSocket {
    // /// unique session id
    // id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    // /// joined room
    // room: String,
    // /// peer name
    // name: Option<String>,
    // /// Chat server
    // addr: Addr<server::ChatServer>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// Helper method that sends ping to client every second.
    /// Also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket::new(), &req, stream);
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

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn users(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String, ()> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

async fn my_async_delay_handler() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}

// ---
#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
#[get("/q")]
async fn query(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
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
            .service(users)
            .service(query)
            .service(web::scope("/users").service(hello))
            .service(fs::Files::new("/{tail:.*}", "./static").index_file("index.html"))
            .route("/async/", web::get().to(my_async_delay_handler))
            .route("/ws/", web::get().to(ws))
            .route("/{name}", web::get().to(index))
            .route("/", web::get().to(index))
    })
        .bind(format!("{}:{}", server_url, server_port))?
        .run()
        .await
}
