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
pub struct Paginator {
    /// Total pages count
    #[serde(rename = "num_pages", skip_serializing_if = "Option::is_none")]
    pub num_pages: Option<i32>,
}

impl Paginator {
    pub fn new() -> Paginator {
        Paginator {
            num_pages: None,
        }
    }
}


