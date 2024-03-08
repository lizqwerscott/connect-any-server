use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};

use connect_any_server::api::user;
use connect_any_server::init;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let state = init().await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/user/adduser", post(user::add_user))
        .route("/user/devices", get(user::get_user_device))
        .with_state(state);

    // .route("/ws", get(ws_handler))
    // .route("/message/generate", post(post_generate_message))
    // .route("/memory/long/get", post(post_get_memory))
    // .route("/memory/long/store", post(post_store_memory));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:22010")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
