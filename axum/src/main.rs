use axum::{
    extract::{Form, Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::debug;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(hello))
        .route("/json", post(json))
        .route("/form", post(form))
        .route("/path/:user_id/:team_id", get(path));

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
    Html(format!("{}, {}", input.name, input.email))
}

#[derive(Deserialize)]
struct Input {
    name: String,
    email: String,
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
