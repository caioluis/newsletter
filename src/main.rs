use newsletter::startup::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port: u16 = std::env::var("PORT")
        .expect("Failed to read configuration.")
        .parse()
        .expect("Failed to parse port");

    run(SocketAddr::from(([0, 0, 0, 0], port))).await
}
