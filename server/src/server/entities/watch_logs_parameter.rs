use serde::Deserialize;

#[derive(Deserialize)]
pub struct WatchLogsQueryParameter {
    pub namespace: Option<String>,
}