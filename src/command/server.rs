use std::env;

use crate::server::mesh_server::MeshServer;
use crate::server::mesh_server_options::MeshServerOptions;

pub async fn start_mesh_server(config_path: &str) {
    init_logging();

    let options = MeshServerOptions::new(config_path).unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        std::process::exit(1);
    });

    let server: MeshServer = MeshServer::new(options).unwrap_or_else(|err| {
        eprintln!("Error creating mesh server: {}", err);
        std::process::exit(1);
    });

    server.start().await;
}

fn init_logging() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mesh::server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "mesh::server=debug");
    }

    pretty_env_logger::init();
}
