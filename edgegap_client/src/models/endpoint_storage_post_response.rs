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
pub struct EndpointStoragePostResponse {
    /// Name of the Endpoint Storage
    #[serde(rename = "name")]
    pub name: String,
    /// Your access key ID to connect to your S3 Bucket.
    #[serde(rename = "access_key_id")]
    pub access_key_id: String,
    /// Full URL to your S3 Bucket with https or http. It's recommended to not include your bucket name as a subdomain
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Your secret access key to connect to your S3 Bucket. Will be encrypted.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// UTC time of endpoint creation
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// UTC time of endpoint last update
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl EndpointStoragePostResponse {
    pub fn new(name: String, access_key_id: String, endpoint: String, bucket: String) -> EndpointStoragePostResponse {
        EndpointStoragePostResponse {
            name,
            access_key_id,
            endpoint,
            bucket,
            create_time: None,
            last_updated: None,
        }
    }
}


