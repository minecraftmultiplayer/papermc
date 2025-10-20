use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Support {
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl Support {
    pub fn new() -> Support {
        Support {
            end: None,
            status: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SUPPORTED")]
    Supported,
    #[serde(rename = "DEPRECATED")]
    Deprecated,
    #[serde(rename = "UNSUPPORTED")]
    Unsupported,
}

impl Default for Status {
    fn default() -> Status {
        Self::Supported
    }
}
