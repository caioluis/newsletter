use newsletter::{configuration::get_configuration, startup::run};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    run(SocketAddr::from((
        [0, 0, 0, 0],
        configuration.application_port,
    )))
    .await
}
