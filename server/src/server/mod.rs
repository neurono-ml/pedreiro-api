pub mod errors;
pub mod entities;
pub mod routes;

use actix_web::{App, HttpServer, web};

use crate::{
    kuberentes_client::K8sClient,
    server::routes::{
        health_check, make_job_scope
    }
};

pub async fn start_server(k8s_client: K8sClient, host: &str, port: u16) -> anyhow::Result<()> {
    log::info!("Starting server on {host}:{port}");
    let k8s_client_data = web::Data::new(k8s_client);

    HttpServer::new(move || {
        App::new()
            .app_data(k8s_client_data.clone())
            .service(health_check)
            .service(make_job_scope())
    })
    .bind((host, port))?
    .run()
    .await?;

    log::info!("Server finished listen on {host}:{port}");
    Ok(())
}