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
pub struct RelayResponse {
    /// The IP Address of the relay
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The host of the relay
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Box<crate::models::MappedPortResponse>>,
}

impl RelayResponse {
    pub fn new() -> RelayResponse {
        RelayResponse {
            ip: None,
            host: None,
            ports: None,
        }
    }
}

