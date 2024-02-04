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
pub struct HorizontalScalerAppVersionLink {
    /// Name of the linked app of the linked version
    #[serde(rename = "app")]
    pub app: String,
    /// Name of the linked app version.
    #[serde(rename = "app_version")]
    pub app_version: String,
    /// Name of the fleet the app version is linked to.
    #[serde(rename = "fleet")]
    pub fleet: String,
    /// UTC time of link creation
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// UTC time of link last update
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl HorizontalScalerAppVersionLink {
    pub fn new(app: String, app_version: String, fleet: String) -> HorizontalScalerAppVersionLink {
        HorizontalScalerAppVersionLink {
            app,
            app_version,
            fleet,
            create_time: None,
            last_updated: None,
        }
    }
}

