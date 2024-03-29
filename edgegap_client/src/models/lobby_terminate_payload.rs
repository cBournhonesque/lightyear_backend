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
pub struct LobbyTerminatePayload {
    /// Name of the lobby
    #[serde(rename = "name")]
    pub name: String,
}

impl LobbyTerminatePayload {
    pub fn new(name: String) -> LobbyTerminatePayload {
        LobbyTerminatePayload {
            name,
        }
    }
}


