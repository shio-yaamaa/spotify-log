use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommitMetadata {
    pub sha: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitItem {
    pub sha: String,
    pub commit: CommitItemCommit,
    pub parents: Vec<CommitItemParent>,
    pub files: Vec<CommitItemFile>,
}

#[derive(Serialize, Deserialize)]
pub struct CommitItemCommit {
    pub committer: CommitItemCommitCommitter,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitItemCommitCommitter {
    pub name: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitItemParent {
    pub sha: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitItemFile {
    pub filename: String, // e.g. "playlists/1.json"
    pub additions: u16,
    pub deletions: u16,
    pub status: String, // "added" | "removed" | "modified"
}
