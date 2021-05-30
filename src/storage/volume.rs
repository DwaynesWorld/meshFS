use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Volume {
    pub url: String,
}

impl Volume {
    pub fn new(url: String) -> Result<Self, &'static str> {
        Ok(Volume { url })
    }
}

impl ToString for Volume {
    fn to_string(&self) -> String {
        self.url.clone()
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Volume) -> bool {
        self.url == other.url
    }
}
