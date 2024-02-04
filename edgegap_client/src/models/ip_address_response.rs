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
pub struct IpAddressResponse {
    /// Public IP Address
    #[serde(rename = "public_ip")]
    pub public_ip: String,
}

impl IpAddressResponse {
    pub fn new(public_ip: String) -> IpAddressResponse {
        IpAddressResponse {
            public_ip,
        }
    }
}

