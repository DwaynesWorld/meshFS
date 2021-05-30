mod command;
mod server;
mod storage;

use clap::{App, Arg, SubCommand};

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
    let opts = App::new("MeshFS")
        .version("1.0")
        .author("Kyle T. <kthompson713@gmail.com>")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("server").arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .required(true)
                    .takes_value(true)
                    .help("Sets a custom config file"),
            ),
        )
        .subcommand(
            SubCommand::with_name("volume").arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .required(true)
                    .takes_value(true)
                    .help("Sets a custom config file"),
            ),
        )
        .get_matches();

    if let Some(opts) = opts.subcommand_matches("server") {
        let config_path = opts.value_of("config").unwrap();
        command::server::start_mesh_server(config_path).await;
    }

    if let Some(opts) = opts.subcommand_matches("volume") {
        let config_path = opts.value_of("config").unwrap();
        command::volume::start_volume_server(config_path).await;
    }
}
