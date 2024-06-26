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
pub struct SessionGet {
    /// Unique UUID
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Custom ID if Available
    #[serde(rename = "custom_id", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// Current status of the session
    #[serde(rename = "status")]
    pub status: String,
    /// If the session is linked to a ready deployment
    #[serde(rename = "ready")]
    pub ready: bool,
    /// If the session is linked to a deployment
    #[serde(rename = "linked")]
    pub linked: bool,
    /// Type of session created
    #[serde(rename = "kind")]
    pub kind: String,
    /// Count of user this session currently have
    #[serde(rename = "user_count")]
    pub user_count: i32,
    /// App version linked to the session
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i32>,
    /// Session created at
    #[serde(rename = "create_time")]
    pub create_time: String,
    /// Elapsed time
    #[serde(rename = "elapsed")]
    pub elapsed: i32,
    /// Error Detail
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Users in the session
    #[serde(rename = "session_users", skip_serializing_if = "Option::is_none")]
    pub session_users: Option<Vec<crate::models::SessionUser>>,
    /// IPS in the session
    #[serde(rename = "session_ips", skip_serializing_if = "Option::is_none")]
    pub session_ips: Option<Vec<crate::models::SessionUser>>,
    #[serde(rename = "deployment", skip_serializing_if = "Option::is_none")]
    pub deployment: Option<crate::models::Deployment>,
    /// When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url 
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl SessionGet {
    pub fn new(session_id: String, custom_id: String, status: String, ready: bool, linked: bool, kind: String, user_count: i32, app_id: i32, create_time: String, elapsed: i32, deployment: crate::models::Deployment) -> SessionGet {
        SessionGet {
            session_id,
            custom_id: Some(custom_id),
            status,
            ready,
            linked,
            kind,
            user_count,
            app_id: Some(app_id),
            create_time,
            elapsed,
            error: None,
            session_users: None,
            session_ips: None,
            deployment: Some(deployment),
            webhook_url: None,
        }
    }
}


