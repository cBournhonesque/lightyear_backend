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
pub struct ActiveDeploymentTelemetryGetResult {
    /// Unique retrieval key to get the telemetry response.
    #[serde(rename = "retrieval_key")]
    pub retrieval_key: String,
    /// Result sorted by best score. Index 0 is the best one.
    #[serde(rename = "scores")]
    pub scores: Vec<crate::models::ActiveDeploymentTelemetryScore>,
    /// If the score list is incomplete and missing request IDs. Can occur if you request the results before we receive telemetry from every deployment.
    #[serde(rename = "partial_result")]
    pub partial_result: bool,
}

impl ActiveDeploymentTelemetryGetResult {
    pub fn new(retrieval_key: String, scores: Vec<crate::models::ActiveDeploymentTelemetryScore>, partial_result: bool) -> ActiveDeploymentTelemetryGetResult {
        ActiveDeploymentTelemetryGetResult {
            retrieval_key,
            scores,
            partial_result,
        }
    }
}


