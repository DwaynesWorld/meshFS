use std::env;
use warp::Filter;

/// Provides a RESTful web server for managing a distributed file system.
///
/// API will be:
///
/// - `GET /<key>`: returns a redirect to the location of the key's value.
/// - `PUT /<key>`: put data in storage.
/// - `DELETE /<key>`: delete from storage.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mesh::server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "mesh::server=debug");
    }

    pretty_env_logger::init();

    let db = sled::open("/tmp/meshfs").expect("open");
    let routes = routes::all(db).with(warp::log("mesh::server"));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

mod routes {
    use super::handlers;
    use sled::Db;
    use warp::Filter;

    pub fn all(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get(db.clone()).or(put(db.clone())).or(delete(db.clone()))
    }

    /// GET /:key
    pub fn get(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path::param())
            .and(with_db(db))
            .and_then(handlers::get)
    }

    /// PUT /:key with body
    pub fn put(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::put()
            .and(warp::path::param())
            .and(warp::body::aggregate())
            .and(with_db(db))
            .and_then(handlers::put)
    }

    /// DELETE /:key
    pub fn delete(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::delete()
            .and(warp::path::param())
            .and(with_db(db))
            .and_then(handlers::delete)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use sled::Db;
    use std::convert::Infallible;
    use warp::{http::Response, http::StatusCode, Buf};

    pub async fn get(key: String, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("get data for key: {}", key);

        let data = db.get(key).unwrap().unwrap();
        let s = std::str::from_utf8(&data).unwrap();

        let response = Response::builder()
            .status(StatusCode::OK)
            .body(String::from(s));

        Ok(response)
    }

    pub async fn put(key: String, data: impl Buf, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("put data: {} {:?}", key, data.chunk());

        db.insert(key, data.chunk()).unwrap();
        Ok(StatusCode::CREATED)
    }

    pub async fn delete(key: String, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("delete data: {}", key);

        db.remove(key).unwrap();
        Ok(StatusCode::NO_CONTENT)
    }
}
