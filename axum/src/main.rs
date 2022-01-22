mod entity;

use axum::{
    extract::{Form, Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router, Server,
};
use dotenv::dotenv;
use rayon::prelude::*;
use sea_orm::{entity::*, query::*, tests_cfg::cake};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Instant};
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(hello))
        .route("/json", post(json))
        .route("/form", post(form))
        .route("/path/:user_id/:team_id", get(path))
        .route("/sum", get(multi_threaded))
        .route("/db", get(from_db));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("Hello, World!"))
}

// JSON
async fn json(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// Form
async fn form(Form(input): Form<Input>) -> impl IntoResponse {
    Html(format!("{}, {:?}", input.name, input.email))
}

#[derive(Deserialize)]
struct Input {
    name: String,
    email: Option<String>,
}

// Path
async fn path(Path(Params { user_id, team_id }): Path<Params>) -> impl IntoResponse {
    Html(format!("{}, {}", user_id, team_id))
}

#[derive(Deserialize)]
struct Params {
    user_id: u32,
    team_id: u32,
}

// Multi threaded
fn fibonacci(n: u32) -> u32 {
    match n {
        0..=1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

async fn multi_threaded() -> impl IntoResponse {
    let input: [u32; 40] = (1..=40).collect::<Vec<_>>().try_into().unwrap();
    let start = Instant::now();
    let response: u32 = input.par_iter().map(|&i| fibonacci(i)).sum();
    let end = Instant::now();
    info!("{}ms", end.duration_since(start).as_millis());
    response.to_string()
}

async fn from_db() -> impl IntoResponse {
    use entity::users;

    let db: DatabaseConnection = Database::connect("sqlite:db.sqlite").await.unwrap();
    let user: Option<users::Model> = users::Entity::find_by_id(1).one(&db).await.unwrap();

    format!("{:?}", user)
}
