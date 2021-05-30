use crate::storage::volume::Volume;
use hashring::{DefaultHashBuilder, HashRing};
use sled::Db;
use std::sync::Mutex;

pub struct MeshServerProperties {
    pub volumes: Vec<Volume>,
    pub v_ring: Mutex<HashRing<Volume, DefaultHashBuilder>>,
    pub db: Db,
}
