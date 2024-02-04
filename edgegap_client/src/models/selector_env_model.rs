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
pub struct SelectorEnvModel {
    /// The Key to inject in the Deployment Environment Variable for this Selector
    #[serde(rename = "key")]
    pub key: String,
    /// The Value to inject in the Deployment Environment Variable for this Selector
    #[serde(rename = "value")]
    pub value: String,
}

impl SelectorEnvModel {
    pub fn new(key: String, value: String) -> SelectorEnvModel {
        SelectorEnvModel {
            key,
            value,
        }
    }
}


