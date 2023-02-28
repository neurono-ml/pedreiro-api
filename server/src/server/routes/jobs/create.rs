use actix_web::{HttpResponse, web};
use shared::PedreiroJob;

use crate::{kuberentes_client::K8sClient, server::{entities::CreateQueryParameter, errors::InternalServerError}};

pub async fn create_job(
    k8s_client_: web::Data<K8sClient>, query_parameters_: web::Query<CreateQueryParameter>,
    payload: web::Json<PedreiroJob>
) -> actix_web::Result<HttpResponse> {
    let k8s_client = k8s_client_.into_inner();
    let predeiro_job = payload.into_inner();
    let query_parameters = query_parameters_.into_inner();
    
    let namespace = query_parameters.namespace.unwrap_or("default".to_string());
    let dry_run = query_parameters.dry_run.unwrap_or(false);

    let created_job = 
        k8s_client
            .create(&predeiro_job, &namespace, dry_run)
            .await
            .map_err(InternalServerError::from)?;

    let response = HttpResponse::Created().json(created_job);
    Ok(response)
}