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
pub struct ApplicationPost {
    /// The application name
    #[serde(rename = "name")]
    pub name: String,
    /// If the application can be deployed
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// If the telemetry agent is installed on the versions of this app.
    #[serde(rename = "is_telemetry_agent_active", skip_serializing_if = "Option::is_none")]
    pub is_telemetry_agent_active: Option<bool>,
    /// Image base64 string
    #[serde(rename = "image")]
    pub image: String,
}

impl ApplicationPost {
    pub fn new(name: String, is_active: bool, image: String) -> ApplicationPost {
        ApplicationPost {
            name,
            is_active,
            is_telemetry_agent_active: None,
            image,
        }
    }
}


