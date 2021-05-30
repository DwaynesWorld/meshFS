use crate::server::mesh_server_models::{DeleteOptions, ListOptions};
use crate::server::mesh_server_options::MeshServerOptions;
use crate::server::mesh_server_properties::MeshServerProperties;
use crate::storage::blob_properties::BlobProperties;
use rand::prelude::IteratorRandom;
use std::convert::Infallible;
use std::sync::Arc;
use warp::hyper::Body;
use warp::{http::Response, http::StatusCode, Buf, Reply};

pub async fn get_blobs(
    list_opts: ListOptions,
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> Result<impl Reply, Infallible> {
    props
        .db
        .iter()
        .skip(list_opts.offset.unwrap_or(0))
        .take(list_opts.limit.unwrap_or(2))
        .for_each(|r| {
            let (k, v) = r.unwrap();
            println!("k {:?} - v {:?}", k, v);
        });

    Ok(StatusCode::OK)
}

pub async fn get_blob(
    key: String,
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> Result<impl Reply, Infallible> {
    // Get blob properties
    // Set hash in header Content-MD5
    // If record soft deleted or hard deleted return 404
    // make head request first to primary volume & then to replication volumes in random order
    // the first that is found is used for redirect
    // if none found, return 404

    let blob_properties = props.db.get(key).expect("error reading from database");
    if blob_properties.is_none() {
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap();

        return Ok(response);
    }

    let blob_properties = blob_properties.unwrap().to_vec();
    let blob_properties = BlobProperties::from(&blob_properties);

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", blob_properties.primary_volume.url)
        .header("Content-Length", 0)
        .body(Body::empty())
        .unwrap();

    Ok(response)
}

pub async fn create_blob(
    key: String,
    mut blob: impl Buf,
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> Result<impl Reply, Infallible> {
    let bytes = blob.copy_to_bytes(blob.remaining());
    let hash = md5::compute(&bytes);
    let content_md5 = format!("{:x}", hash);
    let path = format!(
        "/{}/{}/{}",
        &content_md5[0..2],
        &content_md5[2..6],
        &content_md5
    );

    println!("path: {}", path);

    let content_length = bytes.len() as u64;

    let mut ring = props.v_ring.lock().unwrap();
    let primary_volume = ring.get(&key).unwrap().to_owned();
    drop(ring);

    let replicated_volumes = props
        .volumes
        .to_owned()
        .into_iter()
        .filter(|v| v.url != primary_volume.url)
        .choose_multiple(&mut rand::thread_rng(), opts.replication.into());

    println!("{}", content_md5);

    // TODO: Call PUT on primary and replicated volumes

    let blob_properties = BlobProperties::new(
        primary_volume,
        replicated_volumes,
        content_md5,
        content_length,
    )
    .unwrap();

    props
        .db
        .insert(key, blob_properties.to_bytes())
        .expect("error inserting into database");
    Ok(StatusCode::CREATED)
}

pub async fn delete_blob(
    key: String,
    delete_opts: DeleteOptions,
    opts: Arc<MeshServerOptions>,
    props: Arc<MeshServerProperties>,
) -> Result<impl Reply, Infallible> {
    println!("debug deleting: {} - {:?}", key, delete_opts);
    if delete_opts.soft.unwrap_or(false) {
        println!("soft deleting: {}", key);
    }

    props.db.remove(key).unwrap();
    Ok(StatusCode::NO_CONTENT)
}
