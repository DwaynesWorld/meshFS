use std::u64;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::volume::Volume;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlobProperties {
    /// The primary volume for the blob data.
    pub primary_volume: Volume,

    /// The replicated volumes for the blob data.
    pub replicated_volumes: Vec<Volume>,

    /// Gets or sets the cache-control value stored for the blob.
    pub cache_control: Option<String>,

    /// Gets or sets the content-disposition value stored for the blob.
    pub content_disposition: Option<String>,

    /// The content-encoding value stored for the blob.
    pub content_encoding: Option<String>,

    /// The content-MD5 value stored for the blob.
    pub content_md5: String,

    /// The content-type value stored for the blob.
    pub content_type: Option<String>,

    /// The creation time for the blob, expressed as a UTC value.
    pub created_time: DateTime<Utc>,

    /// If the blob is deleted, gets the the deletion time for the blob, expressed as a UTC value.
    pub deleted_time: Option<DateTime<Utc>>,

    /// Gets the blob's server-side encryption state.
    pub is_server_encrypted: bool,

    /// Gets the blob's server-side deletion state.
    pub is_deleted: bool,

    /// Gets the the last-modified time for the blob, expressed as a UTC value.
    pub last_modified: DateTime<Utc>,

    /// Gets the size of the blob, in bytes.
    pub length: u64,

    /// If the blob is deleted, this is the time the blob is permenantly deleted.
    pub permanent_delete_time: Option<DateTime<Utc>>,
}

impl BlobProperties {
    pub fn new(
        primary_volume: Volume,
        replicated_volumes: Vec<Volume>,
        content_md5: String,
        content_length: u64,
    ) -> Result<BlobProperties, &'static str> {
        let now = Utc::now();
        Ok(BlobProperties {
            primary_volume,
            replicated_volumes,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_md5,
            content_type: None,
            created_time: now,
            deleted_time: None,
            is_server_encrypted: false,
            is_deleted: false,
            last_modified: now,
            length: content_length,
            permanent_delete_time: None,
        })
    }

    pub fn from(encoded: &Vec<u8>) -> Self {
        bincode::deserialize(&encoded[..]).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
