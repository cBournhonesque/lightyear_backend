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
pub struct BulkSessionDelete {
    /// List of Delete
    #[serde(rename = "sessions")]
    pub sessions: Vec<crate::models::SessionDelete>,
    /// List of Delete Errors
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
}

impl BulkSessionDelete {
    pub fn new(sessions: Vec<crate::models::SessionDelete>, errors: Vec<String>) -> BulkSessionDelete {
        BulkSessionDelete {
            sessions,
            errors,
        }
    }
}


