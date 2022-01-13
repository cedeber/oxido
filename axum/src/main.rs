use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use std::net::SocketAddr;
use tracing::debug;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route("/", get(hello));

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
