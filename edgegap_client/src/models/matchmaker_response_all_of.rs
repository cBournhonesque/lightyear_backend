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
pub struct MatchmakerResponseAllOf {
    /// Name of the Matchmaker.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MatchmakerResponseAllOf {
    pub fn new() -> MatchmakerResponseAllOf {
        MatchmakerResponseAllOf {
            name: None,
        }
    }
}


