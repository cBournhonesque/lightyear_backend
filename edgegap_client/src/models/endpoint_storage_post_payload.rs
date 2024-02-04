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
pub struct EndpointStoragePostPayload {
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
    /// Your secret access key to connect to your S3 Bucket. Will be encrypted.
    #[serde(rename = "secret_access_key")]
    pub secret_access_key: String,
}

impl EndpointStoragePostPayload {
    pub fn new(name: String, access_key_id: String, endpoint: String, bucket: String, secret_access_key: String) -> EndpointStoragePostPayload {
        EndpointStoragePostPayload {
            name,
            access_key_id,
            endpoint,
            bucket,
            secret_access_key,
        }
    }
}


