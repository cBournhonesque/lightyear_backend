use leptos::{server, ServerFnError};
use edgegap::apis::configuration::Configuration;

/// Server Function that returns a connect token so that the client can connect to the server
/// 2. Call the matchmaker service to find which server is available and get the server's address
/// 3. Generate the ConnectToken
#[server(StartDeployment)]
pub async fn start_deployment(client_ip: String) -> Result<edgegap::models::Request, ServerFnError> {
    let configuration = Configuration::new();

    let app_name = "simple_box".to_string();
    let version_name = "v1".to_string();
    // let app_name = std::env::var("APPLICATION").unwrap();
    // let version_name = std::env::var("VERSION").unwrap();
    let mut deploy_request = edgegap::models::DeployModel::new(app_name);
    deploy_request.version_name = Some(version_name);
    deploy_request.ip_list = Some(vec![client_ip]);
    deploy_request.container_log_storage = Some(Box::new(edgegap::models::ContainerLogStorageModel::new(true)));
    deploy_request.webhook_url = Some("http://127.0.0.1:3000/webhooks/deployment".to_string());
    let resp = edgegap::apis::deployments_api::deploy(&configuration, deploy_request).await?;
    dbg!(&resp);

    let request_id = resp.request_id.clone();

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        // listen to a webhook
        let deployment_status = edgegap::apis::deployments_api::deployment_status_get(&configuration, &request_id).await?;
        dbg!(&deployment_status);
        if deployment_status.error || deployment_status.running {
            break;
        }
    }
    Ok(resp)
}

/// Server Function that returns a connect token so that the client can connect to the server
/// 2. Call the matchmaker service to find which server is available and get the server's address
/// 3. Generate the ConnectToken
#[server(StartSession)]
pub async fn start_session(client_ip: String) -> Result<edgegap::models::SessionGet, ServerFnError> {
    let configuration = Configuration::new();

    let app_name = "simple_box".to_string();
    let version_name = "v1".to_string();
    // let app_name = std::env::var("APPLICATION").unwrap();
    // let version_name = std::env::var("VERSION").unwrap();
    let mut session_request = edgegap::models::SessionModel::new(app_name);
    session_request.version_name = Some(version_name);
    session_request.ip_list = Some(vec![client_ip]);
    session_request.webhook_url = Some("http://127.0.0.1:3000/webhooks/session".to_string());

    let resp = edgegap::apis::sessions_api::session_post(&configuration, session_request).await;
    dbg!(&resp);

    let session_id = resp?.session_id.clone();
    dbg!(&session_id);

    loop {
        dbg!("Waiting for session to start");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let session_status = edgegap::apis::sessions_api::get_session(&configuration, &session_id).await;
        dbg!(&session_status);
        let status = session_status?;
        if status.error.is_some() || status.ready {
            return Ok(status);
        }
    }
    Err(ServerFnError::new("Session failed to start"))
}

#[server(
  name=DeploymentWebhook,
  prefix="/webhooks",
  endpoint="deployment",
)]
pub async fn deployment_webhook(request_id: String) -> Result<edgegap::models::Status, ServerFnError> {
    let configuration = Configuration::new();
    dbg!("Received deployment webhook for request: {:?}", &request_id);
    let deployment_status = edgegap::apis::deployments_api::deployment_status_get(&configuration, &request_id).await?;
    dbg!("Received deployment webhook: {:?}", &deployment_status);
    // deployment is ready! prepare connect token, and activate a signal to start showing the game to the client
    Ok(deployment_status)
}

#[server(
    name=SessionWebhook,
    prefix="/webhooks",
    endpoint="session",
)]
pub async fn session_webhook(session_id: String) -> Result<edgegap::models::SessionGet, ServerFnError> {
    let configuration = Configuration::new();
    dbg!("Received session webhook for request: {:?}", &session_id);
    let session_status = edgegap::apis::sessions_api::get_session(&configuration, &session_id).await?;
    dbg!("Received session webhook: {:?}", &session_status);
    // deployment is ready! prepare connect token, and activate a signal to start showing the game to the client
    Ok(session_status)
}