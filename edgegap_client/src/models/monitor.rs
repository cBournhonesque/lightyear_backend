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
pub struct Monitor {
    /// API Name
    #[serde(rename = "name")]
    pub name: String,
    /// API Version
    #[serde(rename = "version")]
    pub version: String,
    /// API Host
    #[serde(rename = "host")]
    pub host: String,
    /// API Host URL
    #[serde(rename = "host_url")]
    pub host_url: String,
    /// API Swagger Specification Location
    #[serde(rename = "spec_url")]
    pub spec_url: String,
    /// API Messages
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// API Errors
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl Monitor {
    pub fn new(name: String, version: String, host: String, host_url: String, spec_url: String) -> Monitor {
        Monitor {
            name,
            version,
            host,
            host_url,
            spec_url,
            messages: None,
            errors: None,
        }
    }
}


