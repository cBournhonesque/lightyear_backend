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
pub struct MatchmakerComponentListResponse {
    /// Number of matchmaker components owned by the user.
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "data")]
    pub data: Box<crate::models::MatchmakerComponentResponse>,
}

impl MatchmakerComponentListResponse {
    pub fn new(count: i32, data: crate::models::MatchmakerComponentResponse) -> MatchmakerComponentListResponse {
        MatchmakerComponentListResponse {
            count,
            data: Box::new(data),
        }
    }
}


