use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectsResponse {
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::ProjectResponse>>,
}

impl ProjectsResponse {
    pub fn new() -> ProjectsResponse {
        ProjectsResponse { projects: None }
    }
}
