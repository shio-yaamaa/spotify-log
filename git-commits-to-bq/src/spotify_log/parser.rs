use lazy_static::lazy_static;
use regex::Regex;
use serde_json;
use std::error::Error;

use crate::spotify_log::defs::{Playlist, TrackRelatedActionType};

lazy_static! {
    // Match with these regex from the top; they are not mutually exclusive
    static ref PLAYLIST_CREATION_RE: Regex = Regex::new(r"^:new: Create :file_folder: ").unwrap();
    static ref PLAYLIST_DELETION_RE: Regex = Regex::new(r"^:negative_squared_cross_mark: Delete :file_folder: ").unwrap();
    static ref PLAYLIST_MODIFICATION_RE: Regex = Regex::new(r"^:pencil2: Modify :file_folder: ").unwrap();
    static ref TRACK_ADDITION_RE: Regex = Regex::new(r"^:new: ").unwrap();
    static ref TRACK_REMOVAL_RE: Regex = Regex::new(r"^:negative_squared_cross_mark: ").unwrap();
    static ref TRACK_TRANSFER_RE: Regex = Regex::new(r"^:truck: ").unwrap();
    static ref TRACK_MODIFICATION_RE: Regex = Regex::new(r"^:pencil2: ").unwrap();
}

// Returns None when the commit is not related to a track
pub fn commit_message_to_track_related_action_type(
    message: &str,
) -> Option<TrackRelatedActionType> {
    if PLAYLIST_CREATION_RE.is_match(message)
        || PLAYLIST_DELETION_RE.is_match(message)
        || PLAYLIST_MODIFICATION_RE.is_match(message)
    {
        return None;
    }
    if TRACK_ADDITION_RE.is_match(message) {
        return Some(TrackRelatedActionType::Addition);
    }
    if TRACK_REMOVAL_RE.is_match(message) {
        return Some(TrackRelatedActionType::Removal);
    }
    if TRACK_TRANSFER_RE.is_match(message) {
        return Some(TrackRelatedActionType::Transfer);
    }
    if TRACK_MODIFICATION_RE.is_match(message) {
        return Some(TrackRelatedActionType::Modification);
    }
    return None;
}

pub fn parse_playlist_snapshot(content: &str) -> Result<Playlist, Box<dyn Error>> {
    let parsed_result: Result<Playlist, serde_json::Error> = serde_json::from_str(content);
    return match parsed_result {
        Ok(playlist) => Ok(playlist),
        Err(e) => Err(Box::new(e)),
    };
}
