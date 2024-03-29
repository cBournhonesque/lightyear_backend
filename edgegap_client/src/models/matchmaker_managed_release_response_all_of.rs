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
pub struct MatchmakerManagedReleaseResponseAllOf {
    /// Name of the matchmaker release configuration to use for this managed release.
    #[serde(rename = "release_config_name")]
    pub release_config_name: String,
}

impl MatchmakerManagedReleaseResponseAllOf {
    pub fn new(release_config_name: String) -> MatchmakerManagedReleaseResponseAllOf {
        MatchmakerManagedReleaseResponseAllOf {
            release_config_name,
        }
    }
}


