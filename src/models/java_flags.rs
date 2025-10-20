use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JavaFlags {
    #[serde(rename = "recommended", skip_serializing_if = "Option::is_none")]
    pub recommended: Option<Vec<String>>,
}

impl JavaFlags {
    pub fn new() -> JavaFlags {
        JavaFlags { recommended: None }
    }
}
