use crate::server::mesh_server_handlers as handlers;
use crate::server::mesh_server_models::*;
use crate::server::mesh_server_options::*;
use crate::server::mesh_server_properties::*;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_blobs(opts.clone(), props.clone())
        .or(get_blob(opts.clone(), props.clone()))
        .or(create_blob(opts.clone(), props.clone()))
        .or(delete_blob(opts.clone(), props.clone()))
}

/// GET v1/blobs
fn get_blobs(
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("v1" / "blobs")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<ListQueryOptions>())
        .and(with_options(opts))
        .and(with_properties(props))
        .and_then(handlers::get_blobs)
}

/// GET v1/blobs/:key
fn get_blob(
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("v1" / "blobs" / String)
        .and(warp::get())
        .and(with_options(opts))
        .and(with_properties(props))
        .and_then(handlers::get_blob)
}

/// PUT v1/blobs/:key with body
fn create_blob(
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("v1" / "blobs" / String)
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_options(opts))
        .and(with_properties(props))
        .and_then(handlers::create_blob)
}

/// DELETE v1/blobs/:key
fn delete_blob(
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("v1" / "blobs" / String)
        .and(warp::delete())
        .and(warp::query::<DeleteQueryOptions>())
        .and(with_options(opts))
        .and(with_properties(props))
        .and_then(handlers::delete_blob)
}

fn with_options(
    opts: Arc<MeshServerOptions>,
) -> impl Filter<Extract = (Arc<MeshServerOptions>,), Error = Infallible> + Clone {
    warp::any().map(move || opts.clone())
}

fn with_properties(
    props: Arc<MeshServerProperties>,
) -> impl Filter<Extract = (Arc<MeshServerProperties>,), Error = Infallible> + Clone {
    warp::any().map(move || props.clone())
}
