use std::collections::BTreeMap;

use k8s_openapi::{api::{batch::v1::{Job, JobSpec}, core::v1::{PodTemplateSpec, PodSpec, Container, VolumeMount, Volume}}, apimachinery::pkg::apis::meta::v1::{ObjectMeta}};

use crate::{PedreiroJob, constants::{JOB_NAME_ANNOTATION, JOB_TYPE_ANNOTATION, PEDREIRO_JOB_TYPE, RESTART_POLICY_NEVER, BUILD_ARG_FLAG, BUILD_CONTEXT_FLAG, DESTINATION_FLAG, DOCKERFILE_FLAG, CACHE_FLAG}, traits::ImageFullName, pedreiro_volume::{PedreiroVolume}};


impl PedreiroJob {
    
    pub fn into_job(&self) -> Job {
        let metadata = extract_object_meta(&self);
        let spec = extract_job_object_spec(&self);

        let job = Job {
            metadata,
            spec: Some(spec),
            ..Job::default()
        };
        job
    }
}


fn extract_object_meta(config: &PedreiroJob) -> ObjectMeta {
    let annotations = extract_annotations(config);

    ObjectMeta {
        annotations: Some(annotations),
        name: Some(config.name.clone()),
        ..ObjectMeta::default()
    }
}

fn extract_job_object_spec(config: &PedreiroJob) -> JobSpec {
    let template = extract_job_template(config);
    let active_deadline_seconds = config.active_deadline_seconds.map(|value| value as i64);
    let backoff_limit = config.maximum_retries.map(|value| value as i32);
    let ttl_seconds_after_finished = config.seconds_to_live_after_finished.map(|value| value as i32);

    JobSpec {
        template,
        active_deadline_seconds,
        backoff_limit,
        ttl_seconds_after_finished,
        ..JobSpec::default()
    }
}

fn extract_annotations(config: &PedreiroJob) -> BTreeMap<String, String> {
    let mut annotations = BTreeMap::new();
    
    annotations.insert(JOB_NAME_ANNOTATION.to_owned(), config.name.clone());
    annotations.insert(JOB_TYPE_ANNOTATION.to_owned(), PEDREIRO_JOB_TYPE.to_owned());

    annotations
}


fn extract_job_template(config: &PedreiroJob) -> PodTemplateSpec {
    let metadata = extract_object_meta(config);
    let spec = extract_pod_spec(config);

    PodTemplateSpec {
        metadata: Some(metadata),
        spec: Some(spec),
    }
}

fn extract_pod_spec(config: &PedreiroJob) -> PodSpec {
    let containers = extract_containers(config);
    let volumes = extract_volumes(config);

    PodSpec {
        affinity: config.affinity.clone(),
        node_selector: config.node_selector.clone(),
        service_account_name: config.service_account_name.clone(),
        restart_policy: Some(RESTART_POLICY_NEVER.to_string()),
        volumes: Some(volumes),
        containers,
        ..PodSpec::default()
    }        
}


fn extract_containers(config: &PedreiroJob) -> Vec<Container> {
    let command =
        config.startup_command.clone()
            .map(|startup_command| vec![startup_command]);
    
    let environment_variables =
        config.environment_variables
            .clone();

    let arguments = extract_main_container_arguments(config);
    let volume_mounts = extract_volume_mounts(config);
    let image = config.builder_image.full_name();

    let main = Container {
        command,
        name: config.name.clone(),
        args: Some(arguments),
        volume_mounts: Some(volume_mounts),
        env: environment_variables,
        resources: config.resources.clone(),
        image: Some(image),
        ..Container::default()
    };

    let containers = vec![main];
    containers
}

fn extract_volumes(config: &PedreiroJob) -> Vec<Volume> {
    config.volumes
        .iter()
        .map(|pedreiro_volume| pedreiro_volume.volume.clone())
        .collect()
}

fn extract_volume_mounts(config: &PedreiroJob) -> Vec<VolumeMount> {
    config.volumes
        .iter()
        .map(extract_volume_mount)
        .collect()
}

fn extract_volume_mount(predreiro_volume: &PedreiroVolume) -> VolumeMount {
    let name = predreiro_volume.volume.name.clone();
    let mount_path = predreiro_volume.mount_path.clone();

    VolumeMount {
        mount_path,
        name,
        ..VolumeMount::default()
    }
}

fn extract_main_container_arguments(config: &PedreiroJob) -> Vec<String> {
    let mut build_arguments = Vec::new();

    build_arguments.push(BUILD_CONTEXT_FLAG.to_owned());
    build_arguments.push(config.build_context.clone());

    build_arguments.push(DOCKERFILE_FLAG.to_owned());
    build_arguments.push(config.dockerfile_path.clone());

    build_arguments.push(DESTINATION_FLAG.to_owned());
    build_arguments.push(config.destination_image.full_name());

    if config.cache.is_some() {
        let cache_parameter = format!("{flag}={value}", flag=CACHE_FLAG.to_owned(), value=config.cache.unwrap().to_string());
        build_arguments.push(cache_parameter);
    }

    for (argument_name, argument_value) in config.build_arguments.iter() {
        let command_line_argument = format!("{argument_name}={argument_value}");
        
        build_arguments.push(BUILD_ARG_FLAG.to_owned());
        build_arguments.push(command_line_argument);
    };
    
    build_arguments
}