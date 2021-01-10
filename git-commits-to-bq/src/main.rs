use dotenv::dotenv;
use std::env;

mod bq_client;
mod converter;
mod github_client;
mod spotify_log;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap();
    let repo_owner = env::var("REPO_OWNER").unwrap();
    let repo_name = env::var("REPO_NAME").unwrap();
    let gcp_access_token = env::var("GCP_ACCESS_TOKEN").unwrap();
    let bq_project_id = env::var("BQ_PROJECT_ID").unwrap();
    let bq_dataset_id = env::var("BQ_DATASET_ID").unwrap();
    let bq_action_table_id = env::var("BQ_ACTION_TABLE_ID").unwrap();
    let bq_track_table_id = env::var("BQ_TRACK_TABLE_ID").unwrap();
    let bq_artist_table_id = env::var("BQ_ARTIST_TABLE_ID").unwrap();

    let github_client = github_client::new(&github_token, &repo_owner, &repo_name);
    let actions_result = spotify_log::fetch_track_related_actions(&github_client).await;
    match actions_result {
        Ok(actions) => {
            println!("{:?}", actions);
            let (action_table_rows, track_table_rows, artist_table_rows) =
                converter::track_related_action_to_table_rows(actions);
            println!("{:?}", action_table_rows);
            println!("{:?}", track_table_rows);
            println!("{:?}", artist_table_rows);

            let bq_client = bq_client::new(&gcp_access_token, &bq_project_id, &bq_dataset_id);

            let actions_insert_result = bq_client
                .insert_rows(&bq_action_table_id, action_table_rows)
                .await;
            if let Err(e) = actions_insert_result {
                println!("Error inserting actions: {}", e);
            }
            let tracks_insert_result = bq_client
                .insert_rows(&bq_track_table_id, track_table_rows)
                .await;
            if let Err(e) = tracks_insert_result {
                println!("Error inserting tracks: {}", e);
            }
            let artists_insert_result = bq_client
                .insert_rows(&bq_artist_table_id, artist_table_rows)
                .await;
            if let Err(e) = artists_insert_result {
                println!("Error inserting artists: {}", e);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
