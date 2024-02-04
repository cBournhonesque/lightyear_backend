use leptos::{server, ServerFnError};
use edgegap::apis::configuration::Configuration;

/// Server Function that returns a connect token so that the client can connect to the server
/// 2. Call the matchmaker service to find which server is available and get the server's address
/// 3. Generate the ConnectToken
#[server(StartDeployment)]
pub async fn start_deployment(client_ip: String) -> Result<edgegap::models::Request, ServerFnError> {
    let mut configuration = Configuration::new();

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

    while true {
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

#[server(
  name=DeploymentWebhook,
  prefix="/webhooks",
  endpoint="deployment",
)]
pub async fn deployment_webhook(request_id: String) -> Result<edgegap::models::Status, ServerFnError> {
    let mut configuration = Configuration::new();
    let deployment_status = edgegap::apis::deployments_api::deployment_status_get(&configuration, &request_id).await?;
    dbg!("Received deployment webhook: {:?}", &deployment_status);

    // deployment is ready! prepare connect token, and activate a signal to start showing the game to the client
    Ok(deployment_status)
}