use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionResponse {
    #[serde(rename = "builds", skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<i32>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::Version>>,
}

impl VersionResponse {
    pub fn new() -> VersionResponse {
        VersionResponse {
            builds: None,
            version: None,
        }
    }
}
