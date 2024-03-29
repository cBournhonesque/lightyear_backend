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
pub struct MatchmakerListResponse {
    /// Number of matchmakers owned by the user.
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "data")]
    pub data: Box<crate::models::MatchmakerResponse>,
}

impl MatchmakerListResponse {
    pub fn new(count: i32, data: crate::models::MatchmakerResponse) -> MatchmakerListResponse {
        MatchmakerListResponse {
            count,
            data: Box::new(data),
        }
    }
}


