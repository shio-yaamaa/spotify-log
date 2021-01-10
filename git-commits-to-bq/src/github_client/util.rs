use crate::github_client::defs::DiffType;

pub fn status_to_diff_type(status: &str) -> DiffType {
    return match status {
        "added" => DiffType::Addition,
        "removed" => DiffType::Deletion,
        "modified" => DiffType::Modification,
        _ => DiffType::Unknown,
    };
}
