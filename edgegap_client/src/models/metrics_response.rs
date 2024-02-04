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
pub struct MetricsResponse {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Box<crate::models::TotalMetricsModel>>,
    #[serde(rename = "cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Box<crate::models::MetricsModel>>,
    #[serde(rename = "mem", skip_serializing_if = "Option::is_none")]
    pub mem: Option<Box<crate::models::MetricsModel>>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<crate::models::NetworkMetricsModel>>,
}

impl MetricsResponse {
    pub fn new() -> MetricsResponse {
        MetricsResponse {
            total: None,
            cpu: None,
            mem: None,
            network: None,
        }
    }
}

