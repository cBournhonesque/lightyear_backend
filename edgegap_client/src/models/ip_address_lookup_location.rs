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
pub struct IpAddressLookupLocation {
    #[serde(rename = "continent", skip_serializing_if = "Option::is_none")]
    pub continent: Option<Box<crate::models::IpAddressLookupLocationContinent>>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<crate::models::IpAddressLookupLocationCountry>>,
    /// Latitude
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,
    /// Longitude
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
}

impl IpAddressLookupLocation {
    pub fn new() -> IpAddressLookupLocation {
        IpAddressLookupLocation {
            continent: None,
            country: None,
            latitude: None,
            longitude: None,
        }
    }
}


