use k8s_openapi::api::batch::v1::{Job, JobStatus};
use async_stream::try_stream;
use kube::{Api, api::ListParams, core::WatchEvent};
use futures::{StreamExt, TryStreamExt, Stream};
use shared::WatchMessage;


use super::K8sClient;

impl K8sClient {
    pub fn watch_job(&self, job_name: &str, namespace: &str) -> impl Stream<Item = anyhow::Result<WatchMessage>>
    {   
        let api = Api::<Job>::namespaced(self.client.clone(), namespace);

        let field_selector = format!("metadata.name={job_name}");
        let list_params = ListParams {
            field_selector: Some(field_selector),
            ..ListParams::default()
        };

        try_stream!{
            let mut status_stream = api.watch(&list_params, "0").await?.boxed();
            while let Some(status) = status_stream.try_next().await? {
                
                let may_be_watch_message = match status {
                    WatchEvent::Added(s) => get_status_info("Added", s.status.unwrap_or_default()),
                    WatchEvent::Modified(s) => get_status_info("Modified", s.status.unwrap_or_default()),
                    WatchEvent::Deleted(s) => get_status_info("Deleted", s.status.unwrap_or_default()),
                    WatchEvent::Error(error) => {
                        let event_name = "Error";
                        let reason = error.reason;
                        let message = error.message;
                        let watch_message = WatchMessage::new(event_name, Some(reason), Some(message), None);
                        Some(watch_message)
                    },
                    _ => continue,
                };

                match may_be_watch_message {
                    Some(watch_message) => {
                        yield watch_message;
                    },
                    None => {
                        continue;
                    }
                }
            }
        }
    }

}

fn get_status_info(event_name: &str, status: JobStatus) -> Option<WatchMessage> {
    let reason = status.conditions.clone()?.last()?.reason.clone();
    let message = status.conditions.clone()?.last()?.message.clone();
    let timestamp = status.conditions.clone()?.last()?.last_transition_time.clone();

    let watch_message = WatchMessage::new(event_name, reason, message, timestamp);
    Some(watch_message)    
}