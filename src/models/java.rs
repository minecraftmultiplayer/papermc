use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Java {
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Box<models::JavaFlags>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::JavaVersion>>,
}

impl Java {
    pub fn new() -> Java {
        Java {
            flags: None,
            version: None,
        }
    }
}
