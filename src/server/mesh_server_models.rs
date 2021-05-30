use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteOptions {
    pub soft: Option<bool>,
}
