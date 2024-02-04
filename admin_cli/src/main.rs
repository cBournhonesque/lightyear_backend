use edgegap::apis::configuration::Configuration;

async fn get_deployment_status() {
    let configuration = Configuration::new();
    let request_id = "bdf66e6882ed".to_string();
    let deployment_status = edgegap::apis::deployments_api::deployment_status_get(&configuration, &request_id).await;
    dbg!(&deployment_status);
}

async fn create_deployment() {
    let configuration = Configuration::new();
    let client_ip = "68.173.153.4".to_string();
    let app_name = "simple_box".to_string();
    let version_name = "demo-app-f9ac01".to_string();
    let mut deploy_request = edgegap::models::DeployModel::new(app_name);
    deploy_request.version_name = Some(version_name);
    deploy_request.ip_list = Some(vec![client_ip]);
    deploy_request.container_log_storage = Some(Box::new(edgegap::models::ContainerLogStorageModel::new(true)));
    let resp = edgegap::apis::deployments_api::deploy(&configuration, deploy_request).await;
    dbg!(&resp);
}


#[tokio::main]
async fn main() {
    // create_deployment().await;
    get_deployment_status().await;
}