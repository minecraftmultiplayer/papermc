use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`get_family`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFamilyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_family_builds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFamilyBuildsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_project1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProject1Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_projects1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjects1Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_version1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersion1Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_version_build`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersionBuildError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_version_builds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersionBuildsError {
    UnknownValue(serde_json::Value),
}

pub async fn get_family(
    configuration: &configuration::Configuration,
    project: &str,
    family: &str,
) -> Result<serde_json::Value, Error<GetFamilyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;
    let p_path_family = family;

    let uri_str = format!(
        "{}/v2/projects/{project}/version_group/{family}",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project),
        family = crate::apis::urlencode(p_path_family)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFamilyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_family_builds(
    configuration: &configuration::Configuration,
    project: &str,
    family: &str,
) -> Result<serde_json::Value, Error<GetFamilyBuildsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;
    let p_path_family = family;

    let uri_str = format!(
        "{}/v2/projects/{project}/version_group/{family}/builds",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project),
        family = crate::apis::urlencode(p_path_family)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFamilyBuildsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_project1(
    configuration: &configuration::Configuration,
    project: &str,
) -> Result<serde_json::Value, Error<GetProject1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;

    let uri_str = format!(
        "{}/v2/projects/{project}",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetProject1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_projects1(
    configuration: &configuration::Configuration,
) -> Result<serde_json::Value, Error<GetProjects1Error>> {
    let uri_str = format!("{}/v2/projects", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetProjects1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_version1(
    configuration: &configuration::Configuration,
    project: &str,
    version: &str,
) -> Result<serde_json::Value, Error<GetVersion1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;
    let p_path_version = version;

    let uri_str = format!(
        "{}/v2/projects/{project}/versions/{version}",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project),
        version = crate::apis::urlencode(p_path_version)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetVersion1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_version_build(
    configuration: &configuration::Configuration,
    project: &str,
    version: &str,
    build: i32,
) -> Result<serde_json::Value, Error<GetVersionBuildError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;
    let p_path_version = version;
    let p_path_build = build;

    let uri_str = format!(
        "{}/v2/projects/{project}/versions/{version}/builds/{build}",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project),
        version = crate::apis::urlencode(p_path_version),
        build = p_path_build
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetVersionBuildError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_version_builds(
    configuration: &configuration::Configuration,
    project: &str,
    version: &str,
) -> Result<serde_json::Value, Error<GetVersionBuildsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_project = project;
    let p_path_version = version;

    let uri_str = format!(
        "{}/v2/projects/{project}/versions/{version}/builds",
        configuration.base_path,
        project = crate::apis::urlencode(p_path_project),
        version = crate::apis::urlencode(p_path_version)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetVersionBuildsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
