use serde::{Serialize, Deserialize};

use crate::{helpers::{generate_name, default_image_tag}, traits::ImageFullName};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BuiltImage {
    #[serde(default="generate_name")]
    repository: String,

    #[serde(default="default_image_tag")]
    tag: String
}

impl ImageFullName for BuiltImage {
    fn full_name(&self) -> String {
        format!("{repository}:{tag}", repository=self.repository, tag=self.tag)
    }
}


impl Default for BuiltImage {
    fn default() -> Self {
        Self { repository: generate_name(), tag: default_image_tag() }
    }
}





