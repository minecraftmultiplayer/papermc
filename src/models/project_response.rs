use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectResponse {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::Project>>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<indexmap::IndexMap<String, Vec<String>>>,
}

impl ProjectResponse {
    pub fn new() -> ProjectResponse {
        ProjectResponse {
            project: None,
            versions: None,
        }
    }
}
