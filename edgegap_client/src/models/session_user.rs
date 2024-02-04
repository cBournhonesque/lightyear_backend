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
pub struct SessionUser {
    /// Ip of the user connected to the Session
    #[serde(rename = "ip")]
    pub ip: String,
    /// Latitude
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// Longitude
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
}

impl SessionUser {
    pub fn new(ip: String) -> SessionUser {
        SessionUser {
            ip,
            latitude: None,
            longitude: None,
        }
    }
}


