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
pub struct MetricsModel {
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "datasets", skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<f32>>,
    #[serde(rename = "timestamps", skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<String>>,
}

impl MetricsModel {
    pub fn new() -> MetricsModel {
        MetricsModel {
            labels: None,
            datasets: None,
            timestamps: None,
        }
    }
}


