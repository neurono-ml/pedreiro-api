mod create;
pub struct K8sClient {
    client: kube::Client,
}

impl K8sClient {
    pub async fn new(accept_invalid_certs: bool) -> anyhow::Result<K8sClient> {
        let mut configuration = kube::config::Config::infer().await?;
        configuration.accept_invalid_certs = accept_invalid_certs;
        
        let client = kube::Client::try_from(configuration)?;

        let k8s_client = K8sClient {
            client
        };

        Ok(k8s_client)
    }

    pub async fn health_check(&self) -> anyhow::Result<Vec<String>> {
        let versions = self.client.list_core_api_versions().await?.versions;
        Ok(versions)
    }
}