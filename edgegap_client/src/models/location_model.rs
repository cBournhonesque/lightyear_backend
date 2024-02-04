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
pub struct LocationModel {
    /// Latitude
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// Longitude
    #[serde(rename = "longitude")]
    pub longitude: f32,
}

impl LocationModel {
    pub fn new(latitude: f32, longitude: f32) -> LocationModel {
        LocationModel {
            latitude,
            longitude,
        }
    }
}


