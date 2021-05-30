use crate::server::mesh_server_options::MeshServerOptions;
use crate::server::mesh_server_properties::MeshServerProperties;
use crate::server::mesh_server_router;
use crate::storage::volume::Volume;
use hashring::HashRing;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

pub struct MeshServer {
    options: Arc<MeshServerOptions>,
    properties: Arc<MeshServerProperties>,
}

impl MeshServer {
    pub fn new(options: MeshServerOptions) -> Result<MeshServer, &'static str> {
        let error = format!(
            "failed to create or open database in {}",
            options.meta_location
        );

        let db = sled::open(&options.meta_location).expect(&error);
        let mut v_ring: HashRing<Volume, _> = HashRing::new();
        let mut volumes = Vec::new();

        for url in &options.volume_urls {
            let volume = Volume::new(url.to_owned()).unwrap();
            volumes.push(volume.clone());
            v_ring.add(volume);
        }

        Ok(MeshServer {
            options: Arc::new(options),
            properties: Arc::new(MeshServerProperties {
                db,
                volumes,
                v_ring: Mutex::new(v_ring),
            }),
        })
    }

    pub async fn start(&self) {
        let addr = Ipv4Addr::from_str(self.options.host.as_str()).unwrap();
        let addr = (addr, self.options.port);

        let routes = mesh_server_router::routes(self.options.clone(), self.properties.clone())
            .with(warp::log("mesh::server"));

        warp::serve(routes).run(addr).await
    }
}
