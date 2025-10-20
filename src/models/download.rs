use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Download {
    #[serde(rename = "checksums", skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Box<models::Checksums>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Download {
    pub fn new() -> Download {
        Download {
            checksums: None,
            name: None,
            size: None,
            url: None,
        }
    }
}
