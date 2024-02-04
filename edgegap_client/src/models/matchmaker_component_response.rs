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
pub struct MatchmakerComponentResponse {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Matchmaker component name. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// Container repository where the component's image is hosted.
    #[serde(rename = "repository")]
    pub repository: String,
    /// Container image to use for this component.
    #[serde(rename = "image")]
    pub image: String,
    /// Tag of the container image to use for this component.
    #[serde(rename = "tag")]
    pub tag: String,
    /// Private repo credentials to use for pulling the image, if applicable.
    #[serde(rename = "credentials")]
    pub credentials: Box<crate::models::ComponentCredentials>,
}

impl MatchmakerComponentResponse {
    pub fn new(created_at: String, updated_at: String, name: String, repository: String, image: String, tag: String, credentials: crate::models::ComponentCredentials) -> MatchmakerComponentResponse {
        MatchmakerComponentResponse {
            created_at,
            updated_at,
            name,
            repository,
            image,
            tag,
            credentials: Box::new(credentials),
        }
    }
}

