use chrono::prelude::*;

pub enum DiffType {
    Addition,
    Deletion,
    Modification,
    Unknown,
}

pub struct Commit {
    pub sha: String,
    pub committer_name: String,
    pub message: String,
    pub datetime: DateTime<Utc>,
    pub files: Vec<CommitFile>,
}

pub struct CommitFile {
    pub filename: String, // e.g. "playlists/1.json"
    pub diff_type: DiffType,
    pub added_line_count: u16,
    pub deleted_line_count: u16,
    pub before: String,
    pub after: String,
}
