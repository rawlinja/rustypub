use std::sync::Arc;

use axum::extract::Path;
use axum::http::HeaderMap;
use axum::{extract::State, http::StatusCode, Json};

use crate::models::{Activity, Actor, AppState, Link, Note, WebFingerResponse};

use tokio::sync::RwLock;

const BASE_URL: &str = "http://localhost:3000";

pub async fn root_handler() -> &'static str {
    "Hello, World!"
}

pub async fn create_note_handler(Json(payload): Json<Note>) -> (StatusCode, HeaderMap, Json<Note>) {
    (StatusCode::OK, HeaderMap::new(), Json(payload))
}

pub async fn actor_handler(Path(username): Path<String>) -> (StatusCode, HeaderMap, Json<Actor>) {
    let base = BASE_URL;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/activity+json".parse().unwrap(),
    );

    let actor = Actor {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        kind: "Person".to_string(),
        id: format!("{base}/actor/{username}"),
        preferred_username: username.clone(),
        inbox: format!("{base}/actor/{username}/inbox"),
        outbox: format!("{base}/actor/{username}/outbox"),
    };

    (StatusCode::OK, headers, Json(actor))
}
pub async fn inbox_post_handler(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(username): Path<String>,
    Json(payload): Json<Activity>,
) -> (StatusCode, HeaderMap, Json<Activity>) {
    println!("Received activity: {:?}", payload);
    let mut state = state.write().await;

    state
        .inboxes
        .entry(username)
        .or_default()
        .push_back(payload.clone());

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/activity+json".parse().unwrap(),
    );

    (StatusCode::OK, headers, Json(payload))
}

pub async fn outbox_get_handler(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(username): Path<String>,
) -> (StatusCode, HeaderMap, Json<Vec<Activity>>) {
    println!("Outbox requested for user: {}", username);

    let state = state.read().await;

    let list = state
        .outboxes
        .get(&username)
        .map(|deque| deque.iter().cloned().collect())
        .unwrap_or_default();

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/activity+json".parse().unwrap(),
    );

    (StatusCode::OK, headers, Json(list))
}

pub async fn inbox_get_handler(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(username): Path<String>,
) -> (StatusCode, HeaderMap, Json<Vec<Activity>>) {
    println!("Inbox requested for user: {}", username);

    let state = state.read().await;

    // Fetch inbox items for this user
    let list = state
        .inboxes
        .get(&username)
        .map(|deque| deque.iter().cloned().collect())
        .unwrap_or_default();

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/activity+json".parse().unwrap(),
    );

    (StatusCode::OK, headers, Json(list))
}

pub async fn outbox_post_handler(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(username): Path<String>,
    Json(payload): Json<Activity>,
) -> (StatusCode, HeaderMap, Json<Activity>) {
    println!("Outbox for user: {:?}", username);

    let mut state = state.write().await;
    state
        .outboxes
        .entry(username.clone())
        .or_default()
        .push_back(payload.clone());

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/activity+json".parse().unwrap(),
    );
    (StatusCode::OK, headers, Json(payload))
}

pub async fn webfinger_handler(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<WebFingerResponse>, axum::http::StatusCode> {
    let resource = params
        .get("resource")
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    if !resource.starts_with("acct:") {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let acc = resource.trim_start_matches("acct:");
    let username = acc
        .split('@')
        .next()
        .ok_or(axum::http::StatusCode::BAD_REQUEST)?;

    let actor_url = format!("{BASE_URL}/actor/{}", username);

    let response = WebFingerResponse {
        subject: resource.clone(),
        links: vec![Link {
            rel: "self".into(),
            kind: Some("application/activity+json".into()),
            href: actor_url,
        }],
    };

    Ok(Json(response))
}
