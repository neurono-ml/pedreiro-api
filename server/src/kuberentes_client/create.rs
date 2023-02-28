use k8s_openapi::api::batch::v1::Job;
use kube::{Api, api::PostParams};
use shared::PedreiroJob;

use super::K8sClient;

impl K8sClient {
    pub async fn create(&self, pedreiro_job: &PedreiroJob, namespace: &str, dry_run: bool) -> anyhow::Result<Job> {
        let api = Api::<Job>::namespaced(self.client.clone(), namespace);
        let k8_object = pedreiro_job.into_job();
        let create_parameters = PostParams {dry_run, ..PostParams::default()};
        
        let created_object = api.create(&create_parameters, &k8_object).await?;

        Ok(created_object)
    }
}