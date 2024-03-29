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
pub struct PatchSessionModel {
    /// The List of IP of your user, Array of String, example:     [\"162.254.103.13\",\"198.12.116.39\", \"162.254.135.39\", \"162.254.129.34\"]
    #[serde(rename = "ip_list")]
    pub ip_list: Vec<String>,
}

impl PatchSessionModel {
    pub fn new(ip_list: Vec<String>) -> PatchSessionModel {
        PatchSessionModel {
            ip_list,
        }
    }
}


