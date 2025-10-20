use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildResponse {
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    #[serde(rename = "commits", skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<models::Commit>>,
    #[serde(rename = "downloads", skip_serializing_if = "Option::is_none")]
    pub downloads: Option<indexmap::IndexMap<String, models::Download>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl BuildResponse {
    pub fn new() -> BuildResponse {
        BuildResponse {
            channel: None,
            commits: None,
            downloads: None,
            id: None,
            time: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "ALPHA")]
    Alpha,
    #[serde(rename = "BETA")]
    Beta,
    #[serde(rename = "STABLE")]
    Stable,
    #[serde(rename = "RECOMMENDED")]
    Recommended,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::Alpha
    }
}
