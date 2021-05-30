use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub struct MeshServerOptions {
    pub host: String,
    pub port: u16,
    pub meta_location: String,
    pub volume_urls: Vec<String>,
    pub replication: u16,
}

impl MeshServerOptions {
    pub fn new(config_path: &str) -> Result<MeshServerOptions, &'static str> {
        let mut s = String::new();
        let mut f = File::open(config_path).unwrap();
        f.read_to_string(&mut s).unwrap();

        let docs = YamlLoader::load_from_str(&s).expect("Unable to load yaml configuration file.");
        let doc = &docs[0];

        let host = doc["host"]
            .as_str()
            .unwrap_or_else(|| "localhost")
            .to_owned();

        let port = doc["port"]
            .as_str()
            .unwrap_or_else(|| "3000")
            .parse::<u16>()
            .expect("invalid port");

        let meta_location = doc["meta_location"]["path"]
            .as_str()
            .unwrap_or_else(|| "/tmp/meshfs")
            .to_owned();

        let mut volume_urls = Vec::new();
        for v in doc["volumes"].as_vec().unwrap() {
            volume_urls.push(String::from(v.as_str().unwrap()))
        }

        let replication = doc["replication"]
            .as_str()
            .unwrap_or_else(|| "2")
            .parse::<u16>()
            .expect("invalid replication value");

        Ok(MeshServerOptions {
            host,
            port,
            meta_location,
            volume_urls,
            replication,
        })
    }
}
