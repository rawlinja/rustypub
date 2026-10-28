mod handlers;
mod models;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use handlers::*;
use models::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(tokio::sync::RwLock::new(AppState {
        inboxes: std::collections::HashMap::new(),
        outboxes: std::collections::HashMap::new(),
    }));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/notes", post(create_note_handler))
        .route("/actor/:username", get(actor_handler))
        .route(
            "/actor/:username/outbox",
            get(outbox_get_handler).post(outbox_post_handler),
        )
        .route(
            "/actor/:username/inbox",
            get(inbox_get_handler).post(inbox_post_handler),
        )
        .route("/.well-known/webfinger", get(webfinger_handler))
        .with_state(state);

    let addr: std::net::SocketAddr = "127.0.0.1:3000".parse().unwrap();

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
