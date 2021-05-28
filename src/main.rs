mod server;

use crate::server::mesh_server::MeshServer;
use crate::server::mesh_server_options::MeshServerOptions;
use std::env;

/// Provides a RESTful web server for managing a distributed file system.
///
/// API spec:
///
/// - `GET v1/blobs/`: returns the key and location of all blobs.
/// - `GET v1/blobs/<key>`: returns a redirect to the volume location of the blob.
/// - `PUT v1/blobs/<key>`: put blob data in storage.
/// - `DELETE v1/blobs/<key>`: delete blob from storage.
#[tokio::main]
async fn main() {
    init_logging();

    let options = MeshServerOptions::new().unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        std::process::exit(1);
    });

    let mesh_server: MeshServer = MeshServer::new(options).unwrap_or_else(|err| {
        eprintln!("Error creating mesh server: {}", err);
        std::process::exit(1);
    });

    mesh_server.start().await;
}

fn init_logging() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mesh::server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "mesh::server=debug");
    }

    pretty_env_logger::init();
}
