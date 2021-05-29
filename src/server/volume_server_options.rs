use std::{fs::File, io::Read};

use yaml_rust::YamlLoader;

pub struct VolumeServerOptions {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub location: String,
}

impl VolumeServerOptions {
    pub fn new(config_path: &str) -> Result<VolumeServerOptions, &'static str> {
        let mut s = String::new();
        let mut f = File::open(config_path).unwrap();
        f.read_to_string(&mut s).unwrap();

        let docs = YamlLoader::load_from_str(&s).expect("Unable to load yaml configuration file.");
        let doc = &docs[0];

        let name = doc["name"]
            .as_str()
            .expect("volume name not found")
            .to_owned();

        let host = doc["host"]
            .as_str()
            .unwrap_or_else(|| "localhost")
            .to_owned();

        let port = doc["port"]
            .as_str()
            .unwrap_or_else(|| "3000")
            .parse::<u16>()
            .expect("invalid port");

        let location = doc["location"]["path"]
            .as_str()
            .expect("volume location not found")
            .to_owned();

        Ok(VolumeServerOptions {
            name,
            host,
            port,
            location,
        })
    }
}
