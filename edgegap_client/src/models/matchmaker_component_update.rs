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
pub struct MatchmakerComponentUpdate {
    /// Matchmaker component name. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Container repository where the component's image is hosted.
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Container image to use for this component.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Tag of the container image to use for this component.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Private repo credentials to use for pulling the image, if applicable.
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<crate::models::ComponentCredentials>>,
}

impl MatchmakerComponentUpdate {
    pub fn new() -> MatchmakerComponentUpdate {
        MatchmakerComponentUpdate {
            name: None,
            repository: None,
            image: None,
            tag: None,
            credentials: None,
        }
    }
}

