use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaVersion {
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
}

impl JavaVersion {
    pub fn new() -> JavaVersion {
        JavaVersion { minimum: None }
    }
}
