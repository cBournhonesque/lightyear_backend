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
pub struct AppVersionUpdateSessionConfig {
    /// The kind of session to create. If 'Default' if chosen, the application will be handled like a normal application. The kind of session must be: Default, Seat, Match
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// The number of game slots on each deployment of this app version.
    #[serde(rename = "sockets", skip_serializing_if = "Option::is_none")]
    pub sockets: Option<i32>,
    /// If a deployment should be made autonomously if there is not enough sockets open on a new session.
    #[serde(rename = "autodeploy", skip_serializing_if = "Option::is_none")]
    pub autodeploy: Option<bool>,
    /// The number of minutes a deployment of this app version can spend with no session connected before being terminated.
    #[serde(rename = "empty_ttl", skip_serializing_if = "Option::is_none")]
    pub empty_ttl: Option<i32>,
    /// The number of minutes after a session-type deployment has been terminated to remove all the session information connected to your deployment. Minimum and default value is set to 60 minutes so you can manage your session termination before it is removed.
    #[serde(rename = "session_max_duration", skip_serializing_if = "Option::is_none")]
    pub session_max_duration: Option<i32>,
}

impl AppVersionUpdateSessionConfig {
    pub fn new() -> AppVersionUpdateSessionConfig {
        AppVersionUpdateSessionConfig {
            kind: None,
            sockets: None,
            autodeploy: None,
            empty_ttl: None,
            session_max_duration: None,
        }
    }
}

/// The kind of session to create. If 'Default' if chosen, the application will be handled like a normal application. The kind of session must be: Default, Seat, Match
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Seat")]
    Seat,
    #[serde(rename = "Match")]
    _Match,
}

