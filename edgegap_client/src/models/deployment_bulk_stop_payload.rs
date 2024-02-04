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
pub struct DeploymentBulkStopPayload {
    /// Filters used to match with deployments
    #[serde(rename = "filters")]
    pub filters: Vec<crate::models::DeploymentBulkStopFiltersPayload>,
}

impl DeploymentBulkStopPayload {
    pub fn new(filters: Vec<crate::models::DeploymentBulkStopFiltersPayload>) -> DeploymentBulkStopPayload {
        DeploymentBulkStopPayload {
            filters,
        }
    }
}

