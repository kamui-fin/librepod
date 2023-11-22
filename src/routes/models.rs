use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
