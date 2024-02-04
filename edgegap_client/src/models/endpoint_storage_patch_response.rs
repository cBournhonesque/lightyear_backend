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
pub struct EndpointStoragePatchResponse {
    /// Name of the Endpoint Storage
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Your access key ID to connect to your S3 Bucket.
    #[serde(rename = "access_key_id", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// Full URL to your S3 Bucket with https or http. It's recommended to not include your bucket name as a subdomain
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Your secret access key to connect to your S3 Bucket. Will be encrypted.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// UTC time of endpoint storage creation
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// UTC time of endpoint storage last update
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl EndpointStoragePatchResponse {
    pub fn new() -> EndpointStoragePatchResponse {
        EndpointStoragePatchResponse {
            name: None,
            access_key_id: None,
            endpoint: None,
            bucket: None,
            create_time: None,
            last_updated: None,
        }
    }
}


