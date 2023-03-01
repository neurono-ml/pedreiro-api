use actix_web::{HttpResponse, web::{self, Bytes}, http::{header::ContentType}};
use async_stream::try_stream;
use futures::{pin_mut, StreamExt, Stream};
use serde_json::json;
use shared::WatchMessage;

use crate::{kuberentes_client::K8sClient, server::{entities::WatchLogsQueryParameter, errors::InternalServerError}};

pub async fn watch_job(
    k8s_client_: web::Data<K8sClient>,
    query_parameters_: web::Query<WatchLogsQueryParameter>,
    path_parameters_: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let k8s_client = k8s_client_.into_inner();
    let query_parameters = query_parameters_.into_inner();
    let name = path_parameters_.into_inner();
    
    let namespace = query_parameters.namespace.unwrap_or("default".to_string());

    let result_message_stream = 
        k8s_client
            .watch_job(&name, &namespace);

    let bytes_stream = into_bytes_stream(result_message_stream);
    
    let response = HttpResponse::Ok().content_type(ContentType::json()).streaming(bytes_stream);
    Ok(response)
}


fn into_bytes_stream(source_stream: impl Stream<Item = anyhow::Result<WatchMessage>>) -> impl Stream<Item=Result<Bytes, actix_web::error::Error>>
{
    try_stream! {
        pin_mut!(source_stream);
        
        let log_start_message = json!({
            "message": "Event collection started"
        });

        let json_message = serde_json::to_string(&log_start_message).map_err(InternalServerError::from)?;
        let bytes = Bytes::from(json_message);
        yield bytes;

        while let Some(result_message) = source_stream.next().await {
            match result_message {
                Ok(message) => {
                    let json_message = serde_json::to_string(&message).map_err(InternalServerError::from)?;
                    let bytes = Bytes::from(json_message);
                    yield bytes;
                },
                Err(error) => {
                    let server_error = InternalServerError::from(error);
                    log::error!("An error ocurred while watching job: {server_error}");
                    continue;
                }
            }
        }
    }
}