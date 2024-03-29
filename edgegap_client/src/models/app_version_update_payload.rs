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
pub struct AppVersionUpdatePayload {
    /// The Version Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the Version is active currently in the system
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// The Repository where the image is (i.e. 'harbor.edgegap.com' or 'docker.io')
    #[serde(rename = "docker_repository", skip_serializing_if = "Option::is_none")]
    pub docker_repository: Option<String>,
    /// The name of your image (i.e. 'edgegap/demo')
    #[serde(rename = "docker_image", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    /// The tag of your image (i.e. '0.1.2')
    #[serde(rename = "docker_tag", skip_serializing_if = "Option::is_none")]
    pub docker_tag: Option<String>,
    /// The username to access the docker repository
    #[serde(rename = "private_username", skip_serializing_if = "Option::is_none")]
    pub private_username: Option<String>,
    /// The Private Password or Token of the username (We recommend to use a token)
    #[serde(rename = "private_token", skip_serializing_if = "Option::is_none")]
    pub private_token: Option<String>,
    /// Units of vCPU needed (1024 = 1vcpu)
    #[serde(rename = "req_cpu", skip_serializing_if = "Option::is_none")]
    pub req_cpu: Option<i32>,
    /// Units of memory in MB needed (1024 = 1GB)
    #[serde(rename = "req_memory", skip_serializing_if = "Option::is_none")]
    pub req_memory: Option<i32>,
    /// Units of GPU needed (1024= 1 GPU)
    #[serde(rename = "req_video", skip_serializing_if = "Option::is_none")]
    pub req_video: Option<i32>,
    /// The Max duration of the game
    #[serde(rename = "max_duration", skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,
    /// Allow to inject ASA Variables
    #[serde(rename = "use_telemetry", skip_serializing_if = "Option::is_none")]
    pub use_telemetry: Option<bool>,
    /// Allow to inject Context Variables
    #[serde(rename = "inject_context_env", skip_serializing_if = "Option::is_none")]
    pub inject_context_env: Option<bool>,
    /// ACL Protection is active
    #[serde(rename = "whitelisting_active", skip_serializing_if = "Option::is_none")]
    pub whitelisting_active: Option<bool>,
    /// Allow faster deployment by caching your container image in every Edge site
    #[serde(rename = "force_cache", skip_serializing_if = "Option::is_none")]
    pub force_cache: Option<bool>,
    /// Start of the preferred interval for caching your container
    #[serde(rename = "cache_min_hour", skip_serializing_if = "Option::is_none")]
    pub cache_min_hour: Option<i32>,
    /// End of the preferred interval for caching your container
    #[serde(rename = "cache_max_hour", skip_serializing_if = "Option::is_none")]
    pub cache_max_hour: Option<i32>,
    /// Estimated maximum time in seconds to deploy, after this time we will consider it not working and retry.
    #[serde(rename = "time_to_deploy", skip_serializing_if = "Option::is_none")]
    pub time_to_deploy: Option<i32>,
    /// Enable every location available. By enabling this, your request will use every potential location, including those which may require a longer time to deploy. This means that your application may take up to 2 minutes before being up and ready. This functionality does not support ACL and Caching at the moment.
    #[serde(rename = "enable_all_locations", skip_serializing_if = "Option::is_none")]
    pub enable_all_locations: Option<bool>,
    #[serde(rename = "session_config", skip_serializing_if = "Option::is_none")]
    pub session_config: Option<Box<crate::models::AppVersionUpdateSessionConfig>>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<crate::models::AppVersionPort>>,
    #[serde(rename = "probe", skip_serializing_if = "Option::is_none")]
    pub probe: Option<Box<crate::models::AppVersionProbe>>,
    #[serde(rename = "envs", skip_serializing_if = "Option::is_none")]
    pub envs: Option<Vec<crate::models::AppVersionEnv>>,
    /// Termination grace period in seconds after the SIGTERM signal has been sent
    #[serde(rename = "termination_grace_period_seconds", skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i32>,
    /// The name of the endpoint storage to link
    #[serde(rename = "endpoint_storage", skip_serializing_if = "Option::is_none")]
    pub endpoint_storage: Option<String>,
}

impl AppVersionUpdatePayload {
    pub fn new() -> AppVersionUpdatePayload {
        AppVersionUpdatePayload {
            name: None,
            is_active: None,
            docker_repository: None,
            docker_image: None,
            docker_tag: None,
            private_username: None,
            private_token: None,
            req_cpu: None,
            req_memory: None,
            req_video: None,
            max_duration: None,
            use_telemetry: None,
            inject_context_env: None,
            whitelisting_active: None,
            force_cache: None,
            cache_min_hour: None,
            cache_max_hour: None,
            time_to_deploy: None,
            enable_all_locations: None,
            session_config: None,
            ports: None,
            probe: None,
            envs: None,
            termination_grace_period_seconds: None,
            endpoint_storage: None,
        }
    }
}


