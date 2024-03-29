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
pub struct Pagination {
    /// Current page number
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Next page number
    #[serde(rename = "next_page_number", skip_serializing_if = "Option::is_none")]
    pub next_page_number: Option<i32>,
    /// Previous page number
    #[serde(rename = "previous_page_number", skip_serializing_if = "Option::is_none")]
    pub previous_page_number: Option<i32>,
    #[serde(rename = "paginator", skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Box<crate::models::Paginator>>,
    /// If there is a next page
    #[serde(rename = "has_next", skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// If there is a previous page
    #[serde(rename = "has_previous", skip_serializing_if = "Option::is_none")]
    pub has_previous: Option<bool>,
}

impl Pagination {
    pub fn new() -> Pagination {
        Pagination {
            number: None,
            next_page_number: None,
            previous_page_number: None,
            paginator: None,
            has_next: None,
            has_previous: None,
        }
    }
}


