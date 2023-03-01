use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct WatchMessage {
    event_name: String,
    reason: Option<String>,
    message: Option<String>,
    timestamp: Option<Time>,
}

impl WatchMessage {
    pub fn new(event_name: &str, reason: Option<String>, message: Option<String>, timestamp: Option<Time>) -> WatchMessage {
        WatchMessage {
            event_name: event_name.to_owned(),
            reason,
            message,
            timestamp,
        }
    }
}