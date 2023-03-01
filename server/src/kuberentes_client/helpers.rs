use k8s_openapi::api::core::v1::Pod;
use kube::{Api, api::{Log, ListParams}};


pub async fn load_pods_from_job_name(api: Api<Pod>, job_name: &str) -> anyhow::Result<Vec<Pod>> {
    let label_selector = format!("job_name={job_name}");

    let list_parameters = ListParams {
        label_selector: Some(label_selector),
        ..ListParams::default()
    };

    let pods = api.list(&list_parameters).await?;

    let pod_names = pods.iter().map(|pod|{
        
    })
    .collect();

    todo!()
}
