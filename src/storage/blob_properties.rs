use chrono::{Datelike, Timelike, Utc};

pub struct BlobProperties {
    /// Gets or sets the cache-control value stored for the blob.
    pub cache_control: String,

    /// Gets or sets the content-disposition value stored for the blob.
    pub content_disposition: String,

    /// The content-encoding value stored for the blob.
    pub content_encoding: String,

    /// The content-MD5 value stored for the blob.
    pub content_md5: String,

    /// The content-type value stored for the blob.
    pub content_type: String,

    /// The creation time for the blob, expressed as a UTC value.
    pub created_time: Utc,

    /// If the blob is deleted, gets the the deletion time for the blob, expressed as a UTC value.
    pub deleted_time: String,

    /// Gets the blob's server-side encryption state.
    pub is_server_encrypted: String,

    /// Gets the the last-modified time for the blob, expressed as a UTC value.
    pub last_modified: String,

    /// Gets the size of the blob, in bytes.
    pub length: String,

    /// If the blob is an soft-deleted, gets the number of remaining days before the blob is permenantly deleted.
    pub remaining_days_before_permanent_delete: String,
}
