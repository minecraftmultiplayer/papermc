use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "java", skip_serializing_if = "Option::is_none")]
    pub java: Option<Box<models::Java>>,
    #[serde(rename = "support", skip_serializing_if = "Option::is_none")]
    pub support: Option<Box<models::Support>>,
}

impl Version {
    pub fn new() -> Version {
        Version {
            id: None,
            java: None,
            support: None,
        }
    }
}
