use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub struct MeshServerOptions {
    pub host: String,
    pub port: u16,
    pub meta_folder: String,
    pub volumes: Vec<String>,
}

impl MeshServerOptions {
    pub fn new() -> Result<MeshServerOptions, &'static str> {
        // let args: Vec<_> = env::args().collect();
        let mut s = String::new();
        let mut f = File::open("/Users/kt/dev/personal/meshFS/examples/mesh.yml").unwrap();
        f.read_to_string(&mut s).unwrap();

        let docs = YamlLoader::load_from_str(&s).expect("Unable to load yaml configuration file.");
        let doc = &docs[0];

        let host = doc["host"].as_str().unwrap_or_else(|| "localhost");
        let host = String::from(host);

        let port = doc["port"].as_str().unwrap_or_else(|| "3000");
        let port: u16 = port
            .parse::<u16>()
            .expect(format!("Invalid port: {}", port).as_str());

        let meta_folder = doc["meta_database"]["path"]
            .as_str()
            .unwrap_or_else(|| "/tmp/meshfs");
        let meta_folder = String::from(meta_folder);

        let mut volumes = Vec::new();
        for v in doc["volumes"].as_vec().unwrap() {
            volumes.push(String::from(v.as_str().unwrap()))
        }

        Ok(MeshServerOptions {
            host,
            port,
            meta_folder,
            volumes,
        })
    }
}
