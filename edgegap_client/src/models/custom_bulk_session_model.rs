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
pub struct CustomBulkSessionModel {
    /// Custom Session ID
    #[serde(rename = "custom_id")]
    pub custom_id: String,
    /// The List of IP of your user, Array of String, example:     [\"162.254.103.13\",\"198.12.116.39\", \"162.254.135.39\", \"162.254.129.34\"]
    #[serde(rename = "ip_list")]
    pub ip_list: Vec<String>,
    /// When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url 
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl CustomBulkSessionModel {
    pub fn new(custom_id: String, ip_list: Vec<String>) -> CustomBulkSessionModel {
        CustomBulkSessionModel {
            custom_id,
            ip_list,
            webhook_url: None,
        }
    }
}


