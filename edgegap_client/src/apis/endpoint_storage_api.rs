/*
 * Codema
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@edgegap.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `endpoint_create`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointCreateError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `endpoint_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointDeleteError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `endpoint_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointGetError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `endpoint_update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointUpdateError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `endpoints_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointsListError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_create`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileCreateError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileDeleteError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileGetError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_link_app_version`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileLinkAppVersionError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileListError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_unlink_app_version`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileUnlinkAppVersionError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `pull_profile_update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullProfileUpdateError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Create an endpoint storage to store your container logs at the end of a deployment.
pub async fn endpoint_create(configuration: &configuration::Configuration, payload: crate::models::EndpointStoragePostPayload) -> Result<crate::models::EndpointStoragePostResponse, Error<EndpointCreateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an endpoint storage. All the application versions linked to it won't be able to store logs anymore.
pub async fn endpoint_delete(configuration: &configuration::Configuration, endpoint_name: &str) -> Result<crate::models::EndpointStorageDeleteResponse, Error<EndpointDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve an endpoint storage. The ```secret_access_key``` won't be displayed.
pub async fn endpoint_get(configuration: &configuration::Configuration, endpoint_name: &str) -> Result<crate::models::EndpointStorageGetResponse, Error<EndpointGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an Endpoint Storage with new specifications.
pub async fn endpoint_update(configuration: &configuration::Configuration, endpoint_name: &str, payload: crate::models::EndpointStoragePatchPayload) -> Result<crate::models::EndpointStoragePatchResponse, Error<EndpointUpdateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all endpoint storage.
pub async fn endpoints_list(configuration: &configuration::Configuration, page: Option<i32>, limit: Option<i32>, x_fields: Option<&str>) -> Result<crate::models::EndpointStorageListResponse, Error<EndpointsListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoints", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_fields {
        local_var_req_builder = local_var_req_builder.header("X-Fields", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a pull profile. Pull profile will upload data from an endpoint storage to a deployment container on boot. You must link the application version to the pull profile first.
pub async fn pull_profile_create(configuration: &configuration::Configuration, endpoint_name: &str, payload: crate::models::PullProfilePostPayload) -> Result<crate::models::PullProfilePostResponse, Error<PullProfileCreateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullProfileCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a pull profile. All the application versions linked won't receive the data upload anymore. It will not delete your endpoint storage.
pub async fn pull_profile_delete(configuration: &configuration::Configuration, endpoint_name: &str, pull_profile_name: &str) -> Result<(), Error<PullProfileDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name), pull_profile_name=crate::apis::urlencode(pull_profile_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PullProfileDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a pull profile and its specifications.
pub async fn pull_profile_get(configuration: &configuration::Configuration, endpoint_name: &str, pull_profile_name: &str) -> Result<crate::models::PullProfileGetResponse, Error<PullProfileGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name), pull_profile_name=crate::apis::urlencode(pull_profile_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullProfileGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Link a pull profile to an app version. Without a link, the pull profile by itself will do nothing.
pub async fn pull_profile_link_app_version(configuration: &configuration::Configuration, endpoint_name: &str, pull_profile_name: &str, app_name: &str, version_name: &str) -> Result<crate::models::PullProfileAppVersionLinkResponse, Error<PullProfileLinkAppVersionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name), pull_profile_name=crate::apis::urlencode(pull_profile_name), app_name=crate::apis::urlencode(app_name), version_name=crate::apis::urlencode(version_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullProfileLinkAppVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all pull profiles of an endpoint storage.
pub async fn pull_profile_list(configuration: &configuration::Configuration, endpoint_name: &str, page: Option<i32>, limit: Option<i32>) -> Result<crate::models::PullProfilesListResponse, Error<PullProfileListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profiles", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullProfileListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unlink a pull profile from an app version. It will not delete the pull profile.
pub async fn pull_profile_unlink_app_version(configuration: &configuration::Configuration, endpoint_name: &str, pull_profile_name: &str, app_name: &str, version_name: &str) -> Result<(), Error<PullProfileUnlinkAppVersionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name), pull_profile_name=crate::apis::urlencode(pull_profile_name), app_name=crate::apis::urlencode(app_name), version_name=crate::apis::urlencode(version_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PullProfileUnlinkAppVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a pull profile with new specifications.
pub async fn pull_profile_update(configuration: &configuration::Configuration, endpoint_name: &str, pull_profile_name: &str, payload: crate::models::PullProfilePatchPayload) -> Result<crate::models::PulloProfilePatchResponse, Error<PullProfileUpdateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}", configuration.base_path, endpoint_name=crate::apis::urlencode(endpoint_name), pull_profile_name=crate::apis::urlencode(pull_profile_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PullProfileUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

