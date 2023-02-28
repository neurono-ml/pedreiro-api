use actix_web::{HttpResponse, get, web};
use serde_json::json;

use crate::kuberentes_client::K8sClient;

#[get("/health")]
pub(super) async fn health_check(k8s_client_: web::Data<K8sClient>) -> actix_web::Result<HttpResponse> {
    let k8s_client = k8s_client_.into_inner();
    
    let response = match k8s_client.health_check().await {
        Ok(versions) => HttpResponse::Ok().json(versions),
        Err(error) => {
            let error_message = format!("An error occurred while calling kuberentes API: {error:?}");
            let response_body = json!({
                "message": error_message
            });
            HttpResponse::InternalServerError().json(response_body)
        }
    };

    Ok(response)
}