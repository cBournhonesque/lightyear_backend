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
pub struct ContextDeploymentTagResponse {
    /// Name of the tag
    #[serde(rename = "name")]
    pub name: String,
    /// UTC time of tag creation
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// UTC time of tag last update
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl ContextDeploymentTagResponse {
    pub fn new(name: String) -> ContextDeploymentTagResponse {
        ContextDeploymentTagResponse {
            name,
            create_time: None,
            last_updated: None,
        }
    }
}


