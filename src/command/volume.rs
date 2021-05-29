use std::env;

use crate::server::volume_server::VolumeServer;
use crate::server::volume_server_options::VolumeServerOptions;

pub async fn start_volume_server(config_path: &str) {
    init_logging();

    let options = VolumeServerOptions::new(config_path).unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        std::process::exit(1);
    });

    let server: VolumeServer = VolumeServer::new(options).unwrap_or_else(|err| {
        eprintln!("Error creating volume server: {}", err);
        std::process::exit(1);
    });

    server.start().await;
}

fn init_logging() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mesh::volume=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "mesh::volume=debug");
    }

    pretty_env_logger::init();
}
