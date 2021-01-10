use std::error::Error;

use crate::github_client::defs::Commit;
use crate::github_client::GithubClient;

mod converter;
pub mod defs;
mod parser;
mod util;

pub async fn fetch_track_related_actions(
    github_client: &GithubClient,
) -> Result<Vec<defs::TrackRelatedAction>, Box<dyn Error>> {
    let commit_shas = github_client.fetch_commit_shas();
    let mut results = vec![];
    for sha in &commit_shas {
        let result = github_client.fetch_commit_by_sha(sha).await;
        results.push(result);
    }
    let commits = results
        .into_iter()
        // Aggregate Results into a single Result
        .collect::<Result<Vec<Commit>, Box<dyn Error>>>()?;
    let actions = commits
        .iter()
        .flat_map(|commit| converter::commit_to_track_related_action(&commit))
        .collect();
    return Ok(actions);
}
