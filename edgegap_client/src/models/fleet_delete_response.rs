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
pub struct FleetDeleteResponse {
    /// Success Message
    #[serde(rename = "message")]
    pub message: String,
}

impl FleetDeleteResponse {
    pub fn new(message: String) -> FleetDeleteResponse {
        FleetDeleteResponse {
            message,
        }
    }
}


