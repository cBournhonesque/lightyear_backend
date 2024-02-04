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
pub struct PullProfilePostPayload {
    /// Name of the pull profile
    #[serde(rename = "name")]
    pub name: String,
    /// Source in the S3 bucket to fetch from
    #[serde(rename = "source")]
    pub source: String,
    /// If the source is a File or a Directory
    #[serde(rename = "source_type")]
    pub source_type: SourceType,
    /// Destination path where your source will be uploaded in your container. Make sure to avoid protected destinations, such as `/etc/`, as this will prevent the files from being copied to your deployment, and will make your deployment fail. Make sure a normal user can write to the destination folder.
    #[serde(rename = "destination")]
    pub destination: String,
}

impl PullProfilePostPayload {
    pub fn new(name: String, source: String, source_type: SourceType, destination: String) -> PullProfilePostPayload {
        PullProfilePostPayload {
            name,
            source,
            source_type,
            destination,
        }
    }
}

/// If the source is a File or a Directory
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Folder")]
    Folder,
}
