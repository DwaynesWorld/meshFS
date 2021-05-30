use std::env;

use crate::server::volume_server::VolumeServer;
use crate::server::volume_server_options::VolumeServerOptions;

pub async fn start_volume_server(config_path: &str) {
    let options = VolumeServerOptions::new(config_path).unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        std::process::exit(1);
    });

    init_logging(&options.name);

    let server: VolumeServer = VolumeServer::new(options).unwrap_or_else(|err| {
        eprintln!("Error creating volume server: {}", err);
        std::process::exit(1);
    });

    server.start().await;
}

fn init_logging(name: &str) {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mesh::volume=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", format!("{}=debug", name));
    }

    pretty_env_logger::init();
}
