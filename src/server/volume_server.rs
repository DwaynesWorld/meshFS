use std::{net::Ipv4Addr, str::FromStr};
use warp::Filter;

use super::volume_server_options::VolumeServerOptions;

pub struct VolumeServer {
    pub options: VolumeServerOptions,
}

impl VolumeServer {
    pub fn new(options: VolumeServerOptions) -> Result<VolumeServer, &'static str> {
        Ok(VolumeServer { options })
    }

    pub async fn start(&self) {
        let routes = router::routes().with(warp::log("mesh::volume"));
        let addr = Ipv4Addr::from_str(self.options.host.as_str()).unwrap();
        let socket_addr = (addr, self.options.port);
        warp::serve(routes).run(socket_addr).await
    }
}

mod router {
    use super::handlers;
    use warp::{Filter, Rejection, Reply};

    pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        get_blob().or(create_blob()).or(delete_blob())
    }

    /// GET v1/blobs/:key
    fn get_blob() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::get()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and_then(handlers::get_blob)
    }

    /// PUT v1/blobs/:key with body
    fn create_blob() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::put()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and(warp::body::aggregate())
            .and_then(handlers::create_blob)
    }

    /// DELETE v1/blobs/:key
    fn delete_blob() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::delete()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and_then(handlers::delete_blob)
    }
}

mod handlers {
    use std::convert::Infallible;
    use warp::{http::Response, http::StatusCode, Buf, Reply};

    pub async fn get_blob(_: String) -> Result<impl Reply, Infallible> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(String::from("somr"));

        Ok(response)
    }

    pub async fn create_blob(_: String, mut blob: impl Buf) -> Result<impl Reply, Infallible> {
        let bytes = blob.copy_to_bytes(blob.remaining());
        let meta_key = md5::compute(&bytes);
        println!("{:x}", meta_key);

        Ok(StatusCode::CREATED)
    }

    pub async fn delete_blob(key: String) -> Result<impl Reply, Infallible> {
        log::debug!("delete blob: {}", key);

        Ok(StatusCode::NO_CONTENT)
    }
}
