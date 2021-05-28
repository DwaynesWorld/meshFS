use crate::server::mesh_server_options::MeshServerOptions;

use sled::Db;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub struct MeshServer {
    options: MeshServerOptions,
    db: Arc<Db>,
}

impl MeshServer {
    pub fn new(options: MeshServerOptions) -> Result<MeshServer, &'static str> {
        let error = format!("failed to create or open database {}", options.meta_folder);
        let db = sled::open(options.meta_folder.as_str()).expect(error.as_str());

        Ok(MeshServer {
            options,
            db: Arc::new(db),
        })
    }

    pub async fn start(&self) {
        let routes = router::routes(self.db.clone()).with(warp::log("mesh::server"));
        let addr = Ipv4Addr::from_str(self.options.host.as_str()).unwrap();
        let socket_addr = (addr, self.options.port);
        warp::serve(routes).run(socket_addr).await
    }
}

mod router {
    use super::handlers;
    use sled::Db;
    use std::convert::Infallible;
    use std::sync::Arc;
    use warp::{Filter, Rejection, Reply};

    pub fn routes(db: Arc<Db>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        get_blobs(db.clone())
            .or(get_blob(db.clone()))
            .or(put_blob(db.clone()))
            .or(delete_blob(db.clone()))
    }

    /// GET v1/blobs
    fn get_blobs(db: Arc<Db>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::get()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::end())
            .and(with_db(db))
            .and_then(handlers::get_blobs)
    }

    /// GET v1/blobs/:key
    fn get_blob(db: Arc<Db>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::get()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and(warp::path::end())
            .and(with_db(db))
            .and_then(handlers::get_blob)
    }

    /// PUT v1/blobs/:key with body
    fn put_blob(db: Arc<Db>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::put()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and(warp::body::aggregate())
            .and(with_db(db))
            .and_then(handlers::put_blob)
    }

    /// DELETE v1/blobs/:key
    fn delete_blob(db: Arc<Db>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::delete()
            .and(warp::path("v1"))
            .and(warp::path("blobs"))
            .and(warp::path::param())
            .and(with_db(db))
            .and_then(handlers::delete_blob)
    }

    fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use sled::Db;
    use std::convert::Infallible;
    use std::sync::Arc;
    use warp::{http::Response, http::StatusCode, Buf, Reply};

    pub async fn get_blobs(db: Arc<Db>) -> Result<impl Reply, Infallible> {
        log::debug!("get list of for key");

        // let pairs: FromIterator<Result<(sled::IVec, sled::IVec), sled::Error>> =
        // db.into_iter().skip(5).take(5).collect();
        // println!("pairs {:?}", pairs);

        Ok(StatusCode::OK)
    }

    pub async fn get_blob(key: String, db: Arc<Db>) -> Result<impl Reply, Infallible> {
        log::debug!("get data for key: {}", key);

        let blob = db.get(key).unwrap().unwrap();
        let s = std::str::from_utf8(&blob).unwrap();

        let response = Response::builder()
            .status(StatusCode::OK)
            .body(String::from(s));

        Ok(response)
    }

    pub async fn put_blob(
        key: String,
        mut blob: impl Buf,
        db: Arc<Db>,
    ) -> Result<impl Reply, Infallible> {
        // log::debug!("put blob: {} {:?}", key, blob.chunk());

        let bytes = blob.copy_to_bytes(blob.remaining());
        let meta_key = md5::compute(&bytes);
        println!("{:x}", meta_key);

        db.insert(key, &bytes[..]).unwrap();
        Ok(StatusCode::CREATED)
    }

    pub async fn delete_blob(key: String, db: Arc<Db>) -> Result<impl Reply, Infallible> {
        log::debug!("delete blob: {}", key);

        db.remove(key).unwrap();
        Ok(StatusCode::NO_CONTENT)
    }
}
