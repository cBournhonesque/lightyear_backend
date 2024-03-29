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
pub struct DeploymentStopResponse {
    /// The id of the request for a deployment
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DeploymentStopResponse {
    pub fn new(request_id: String) -> DeploymentStopResponse {
        DeploymentStopResponse {
            request_id,
        }
    }
}


