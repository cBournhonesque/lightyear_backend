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
pub struct AppVersionWhitelistEntrySuccess {
    /// if the operation succeed
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "whitelist_entry", skip_serializing_if = "Option::is_none")]
    pub whitelist_entry: Option<Box<crate::models::AppVersionWhitelistEntry>>,
}

impl AppVersionWhitelistEntrySuccess {
    pub fn new(success: bool) -> AppVersionWhitelistEntrySuccess {
        AppVersionWhitelistEntrySuccess {
            success,
            whitelist_entry: None,
        }
    }
}


