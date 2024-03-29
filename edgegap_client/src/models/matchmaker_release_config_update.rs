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
pub struct MatchmakerReleaseConfigUpdate {
    /// Matchmaker configuration name. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Matchmaker configuration, parsed as a string.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

impl MatchmakerReleaseConfigUpdate {
    pub fn new() -> MatchmakerReleaseConfigUpdate {
        MatchmakerReleaseConfigUpdate {
            name: None,
            configuration: None,
        }
    }
}


