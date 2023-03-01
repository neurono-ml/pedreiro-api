use k8s_openapi::api::core::v1::Volume;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PedreiroVolume {
    pub(crate) mount_path: String,
    
    #[serde(flatten)]
    pub(crate) volume: Volume
}