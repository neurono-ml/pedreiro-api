use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateQueryParameter {
    pub namespace: Option<String>,
    pub dry_run: Option<bool>,
}