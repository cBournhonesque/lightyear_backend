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
pub struct MatchmakerReleaseResponseBase {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Name of the app to deploy using the matchmaker.
    #[serde(rename = "app_name")]
    pub app_name: String,
    /// Name of the version of the specified app to deploy using the matchmaker.
    #[serde(rename = "version_name")]
    pub version_name: String,
    /// Name of the matchmaker release. Should be unique, and will be used to differentiate your releases.
    #[serde(rename = "version")]
    pub version: String,
}

impl MatchmakerReleaseResponseBase {
    pub fn new(created_at: String, updated_at: String, app_name: String, version_name: String, version: String) -> MatchmakerReleaseResponseBase {
        MatchmakerReleaseResponseBase {
            created_at,
            updated_at,
            app_name,
            version_name,
            version,
        }
    }
}


