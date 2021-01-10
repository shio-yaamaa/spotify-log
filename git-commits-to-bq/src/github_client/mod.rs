use chrono::prelude::*;
use github_rs::client::{Executor, Github};
use std::error::Error;

mod api_response_defs;
pub mod defs;
mod util;

pub struct GithubClient {
    client: Github,
    repo_owner: String,
    repo_name: String,
}

impl GithubClient {
    pub fn fetch_commit_shas(&self) -> Vec<String> {
        let per_page = 100; // The max limit of the API

        // The page number starts from 1, not 0
        let mut shas = self.fetch_commit_shas_recursively(per_page, 1);

        // Commits are in descending order, so reverse the vec to make it ascending
        shas.reverse();
        return shas;
    }

    fn fetch_commit_shas_recursively(&self, per_page: u8, page: u8) -> Vec<String> {
        let commits_endpoint = format!(
            "repos/{}/{}/commits?per_page={}&page={}",
            &self.repo_owner, &self.repo_name, per_page, page
        );
        let res = self
            .client
            .get()
            .custom_endpoint(&commits_endpoint)
            .execute::<Vec<api_response_defs::CommitMetadata>>();
        // When no item is found on the page, the API returns
        // an empty array with status code 200 instead of 404.
        let shas = match res {
            Ok((_headers, _status, data)) => data
                .unwrap_or_default()
                .iter()
                .map(|metadata| metadata.sha.to_string())
                .collect(),
            Err(_e) => Vec::<String>::new(),
        };
        // If the previous page was the last page, stop the recursive calls.
        if shas.is_empty() {
            return shas;
        }
        let shas_in_following_pages = self.fetch_commit_shas_recursively(per_page, page + 1);
        return [shas, shas_in_following_pages].concat();
    }

    pub async fn fetch_commit_by_sha(&self, sha: &str) -> Result<defs::Commit, Box<dyn Error>> {
        let res = self
            .client
            .get()
            .repos()
            .owner(&self.repo_owner)
            .repo(&self.repo_name)
            .commits()
            .sha(sha)
            .execute::<api_response_defs::CommitItem>();
        match res {
            Ok((_headers, _status, data)) => match data {
                Some(commit_response) => {
                    return Ok(self.commit_response_to_commit(&commit_response).await)
                }
                None => {
                    return Err(From::from(
                        "Failed to parse the response from the API endpoint /repos/{owner}/{repo}/commits/{ref}",
                    ));
                }
            },
            Err(e) => return Err(Box::new(e)),
        }
    }

    async fn commit_response_to_commit(
        &self,
        commit_response: &api_response_defs::CommitItem,
    ) -> defs::Commit {
        // The first commit is not a Spotify log, so just ignore it
        if commit_response.parents.is_empty() {
            return defs::Commit {
                sha: commit_response.sha.to_string(),
                committer_name: commit_response.commit.committer.name.to_string(),
                message: commit_response.commit.message.to_string(),
                datetime: commit_response
                    .commit
                    .committer
                    .date
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
                files: vec![],
            };
        }
        let parent_sha = &commit_response.parents[0].sha;
        let mut files = vec![];
        for file_response in &commit_response.files {
            let diff_type = util::status_to_diff_type(&file_response.status);
            let before = match diff_type {
                defs::DiffType::Addition => String::from(""),
                _ => self
                    .fetch_file_content(parent_sha, &file_response.filename)
                    .await
                    .unwrap_or(String::from("")),
            };
            let after = match diff_type {
                defs::DiffType::Deletion => String::from(""),
                _ => self
                    .fetch_file_content(&commit_response.sha, &file_response.filename)
                    .await
                    .unwrap_or(String::from("")),
            };
            let commit_file = defs::CommitFile {
                filename: file_response.filename.to_string(),
                diff_type,
                added_line_count: file_response.additions,
                deleted_line_count: file_response.deletions,
                before,
                after,
            };
            files.push(commit_file);
        }

        return defs::Commit {
            sha: commit_response.sha.to_string(),
            committer_name: commit_response.commit.committer.name.to_string(),
            message: commit_response.commit.message.to_string(),
            datetime: commit_response
                .commit
                .committer
                .date
                .parse::<DateTime<Utc>>()
                .unwrap(),
            files,
        };
    }

    async fn fetch_file_content(
        &self,
        commit_sha: &str,
        path: &str,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://github.com/{}/{}/raw/{}/{}",
            self.repo_owner, self.repo_name, commit_sha, path
        );
        let content = reqwest::get(&url).await?.text().await?;
        return Ok(content);
    }
}

pub fn new(token: &str, repo_owner: &str, repo_name: &str) -> GithubClient {
    return GithubClient {
        client: Github::new(token).unwrap(),
        repo_owner: repo_owner.to_string(),
        repo_name: repo_name.to_string(),
    };
}
