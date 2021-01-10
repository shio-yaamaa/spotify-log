use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::Serialize;
use serde_json;

mod defs;

pub struct BqClient {
    client: reqwest::Client,
    access_token: String,
    project_id: String,
    dataset_id: String,
}

const API_ROOT: &str = "https://bigquery.googleapis.com/bigquery/v2/";

impl BqClient {
    pub async fn insert_rows<T>(
        &self,
        table_id: &str,
        rows: Vec<T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let path = format!(
            "projects/{}/datasets/{}/tables/{}/insertAll",
            self.project_id, self.dataset_id, table_id
        );
        let url = format!("{}{}", API_ROOT, path);
        let request_body = defs::InsertRowsRequestBody {
            rows: rows
                .into_iter()
                .map(|row| defs::InsertRowsRequestBodyRow { json: row })
                .collect(),
        };
        let resp = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .body(serde_json::to_string(&request_body)?)
            .send()
            .await?;
        println!("{:#?}", resp);
        return Ok(());
    }
}

pub fn new(access_token: &str, project_id: &str, dataset_id: &str) -> BqClient {
    return BqClient {
        client: reqwest::Client::new(),
        access_token: access_token.to_string(),
        project_id: project_id.to_string(),
        dataset_id: dataset_id.to_string(),
    };
}
