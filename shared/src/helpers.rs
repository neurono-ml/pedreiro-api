use names::{Generator, Name};

pub(crate) fn generate_name() -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    generator.next().unwrap()
}


pub(crate) fn dockerfile_path() -> String {
    "Dockerfile".to_string()
}

pub(crate) fn default_image_repository() -> String {
    return "gcr.io/kaniko-project/executor".to_string()
}

pub(crate) fn default_image_tag() -> String {
    return "latest".to_string()
}
