use clap::Parser;
use command_line::CommandLine;
use server::start_server;

mod server;
mod command_line;
mod kuberentes_client;

#[tokio::main(flavor="current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let command_line = CommandLine::parse();
    let k8s_client = kuberentes_client::K8sClient::new(command_line.accept_invalid_certificates).await?;
    start_server(k8s_client, &command_line.host, command_line.port).await?;

    Ok(())

}
