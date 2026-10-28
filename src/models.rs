use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    #[serde(rename = "preferredUsername")]
    pub preferred_username: String,
    pub inbox: String,
    pub outbox: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Activity {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    pub object: Value,
}

#[derive(Default)]
pub struct AppState {
    // Add shared state here if needed
    pub inboxes: std::collections::HashMap<String, VecDeque<Activity>>,
    pub outboxes: std::collections::HashMap<String, VecDeque<Activity>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub rel: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebFingerResponse {
    pub subject: String,
    pub links: Vec<Link>,
}
