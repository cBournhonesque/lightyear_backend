use edgegap::apis::configuration::Configuration;

async fn get_deployment_status() {
    let configuration = Configuration::new();
    let request_id = "466161069f07".to_string();
    let deployment_status = edgegap::apis::deployments_api::deployment_status_get(&configuration, &request_id).await;
    dbg!(&deployment_status);
}

async fn post_session() {
    let configuration = Configuration::new();
    let session_id = "session_1".to_string();
    let app_name = "simple_box".to_string();
    let version_name = "v1".to_string();
    let mut session_request = edgegap::models::SessionModel::new(app_name);
    session_request.version_name = Some(version_name);
    // create the session on a specific deployment
    let request_id = "48c96b470279".to_string();
    session_request.deployment_request_id = Some(request_id);
    let deployment_status = edgegap::apis::sessions_api::session_post(&configuration, session_request).await;
    dbg!(&deployment_status);
}

async fn delete_session() {
    let configuration = Configuration::new();
    let session_id = "session_1".to_string();
    let delete_status = edgegap::apis::sessions_api::session_delete(&configuration, &session_id).await;
    dbg!(&delete_status);
}

async fn update_application() {
    let configuration = Configuration::new();
    let app_name = "simple_box".to_string();
    let version_name = "v1".to_string();
    let mut payload = edgegap::models::AppVersionUpdatePayload::new();
    payload.inject_context_env = Some(true);
    let resp = edgegap::apis::applications_api::app_versions_patch(&configuration, &app_name, &version_name, payload).await;
    dbg!(&resp);
}

async fn create_deployment() {
    let configuration = Configuration::new();
    let client_ip = "68.173.153.4".to_string();
    let app_name = "simple_box".to_string();
    let version_name = "v1".to_string();
    let mut deploy_request = edgegap::models::DeployModel::new(app_name);
    deploy_request.version_name = Some(version_name);
    deploy_request.ip_list = Some(vec![client_ip]);
    deploy_request.container_log_storage = Some(Box::new(edgegap::models::ContainerLogStorageModel::new(true)));
    let resp = edgegap::apis::deployments_api::deploy(&configuration, deploy_request).await;
    dbg!(&resp);
}


#[tokio::main]
async fn main() {
    // update_application().await;
    // create_deployment().await;
    get_deployment_status().await;
}