/*
 * Codema
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@edgegap.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLogStorageModel {
    /// Will override the app version container log storage for this deployment
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The name of your endpoint storage. If container log storage is enabled without this parameter, we will try to take the app version endpoint storage. If there is no endpoint storage in your app version, the container logs will not be stored. If we don't find any endpoint storage associated with this name, the container logs will not be stored.
    #[serde(rename = "endpoint_storage", skip_serializing_if = "Option::is_none")]
    pub endpoint_storage: Option<String>,
}

impl ContainerLogStorageModel {
    pub fn new(enabled: bool) -> ContainerLogStorageModel {
        ContainerLogStorageModel {
            enabled,
            endpoint_storage: None,
        }
    }
}


