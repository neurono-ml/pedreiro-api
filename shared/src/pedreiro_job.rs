use std::collections::{HashMap, BTreeMap};

use k8s_openapi::api::core::v1::{EnvVar, ResourceRequirements, Affinity};
use serde::{Serialize, Deserialize};

use crate::{built_image::BuiltImage, helpers::dockerfile_path, pedreiro_image::PedreiroImage, pedreir_volume::PedreiroVolume};
use super::helpers::generate_name;


#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PedreiroJob {
    /// Job name to be deployed
    #[serde(default="generate_name")]
    pub(crate) name: String,
    
    /// Build context from tar file on object storage, local mounted volume or git
    pub(crate) build_context: String,

    /// Relative path to Dockerfile in the context
    #[serde(default="dockerfile_path")]
    pub(crate) dockerfile_path: String,

    /// Result image coordinates
    #[serde(default="BuiltImage::default")]
    pub(crate) destination_image: BuiltImage,
    
    /// Environment variables to set in job
    pub(crate) environment_variables: Option<Vec<EnvVar>>,

    /// Kaniko command line arguments
    #[serde(default="HashMap::new")]
    pub(crate) build_arguments: HashMap<String, String>,
    
    /// Service account to use in build
    pub(crate) service_account_name: Option<String>,

    /// Job restart policy
    pub(crate) restart_policy: Option<String>,

    /// Time before deleting pod after finished
    pub(crate) seconds_to_live_after_finished: Option<u32>,

    /// Maximum retries
    pub(crate) maximum_retries: Option<u32>,

    /// Maximum execution time for the job
    pub(crate) active_deadline_seconds: Option<u64>,

    /// Kaniko executor image coordinates
    #[serde(default="PedreiroImage::default")]
    pub(crate) builder_image: PedreiroImage,

    /// Pod resources configuration
    pub(crate) resources: Option<ResourceRequirements>,

    /// Job node affinity
    pub(crate) affinity: Option<Affinity>,

    /// Job node selector
    pub(crate) node_selector: Option<BTreeMap<String, String>>,

    /// startup command
    pub(crate) startup_command: Option<String>,

    ///Volumes to be mounted
    #[serde(default)]
    pub(crate) volumes: Vec<PedreiroVolume>
    
}

