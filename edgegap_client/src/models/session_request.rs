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
pub struct SessionRequest {
    /// The Unique Identifier of the Session
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Custom ID if Available
    #[serde(rename = "custom_id", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// The Name of the App you requested
    #[serde(rename = "app")]
    pub app: String,
    /// The name of the App Version you requested
    #[serde(rename = "version")]
    pub version: String,
    /// The Name of the App Version you want to deploy, example:    v1.0
    #[serde(rename = "deployment_request_id", skip_serializing_if = "Option::is_none")]
    pub deployment_request_id: Option<String>,
    /// List of Selectors to filter potential Deployment to link and tag the Session
    #[serde(rename = "selectors", skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<crate::models::SelectorModel>>,
    /// When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url 
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl SessionRequest {
    pub fn new(session_id: String, app: String, version: String) -> SessionRequest {
        SessionRequest {
            session_id,
            custom_id: None,
            app,
            version,
            deployment_request_id: None,
            selectors: None,
            webhook_url: None,
        }
    }
}


