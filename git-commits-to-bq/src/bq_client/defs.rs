use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InsertRowsRequestBody<T> {
    pub rows: Vec<InsertRowsRequestBodyRow<T>>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertRowsRequestBodyRow<T> {
    pub json: T,
}
