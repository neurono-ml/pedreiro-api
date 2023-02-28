use crate::{helpers::{default_image_repository, default_image_tag}, traits::ImageFullName};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct PedreiroImage {
    /// Repository image
    #[serde(default="default_image_repository")]
    repository: String,

    /// Repository tag
    #[serde(default="default_image_tag")]
    tag: String
}


impl Default for PedreiroImage {
    fn default() -> Self {
        Self { repository: default_image_repository(), tag: default_image_tag() }
    }
}

impl ImageFullName for PedreiroImage {
    fn full_name(&self) -> String {
        format!("{repository}:{tag}", repository=self.repository, tag=self.tag)
    }
}