use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListQueryOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteQueryOptions {
    pub soft: Option<bool>,
}
