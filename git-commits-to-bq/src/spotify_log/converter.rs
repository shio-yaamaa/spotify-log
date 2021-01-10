use chrono::prelude::*;
use lazy_static::lazy_static;

use crate::github_client;
use crate::spotify_log::defs::{TrackRelatedAction, TrackRelatedActionType};
use crate::spotify_log::parser;
use crate::spotify_log::util;

const LOG_COMMITTER_NAME: &str = "GitHub Actions";

lazy_static! {
    static ref LOG_STARTED_AT: DateTime<Utc> =
        "2019-10-03T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
}

// Returns None when the commit is not related to a track
pub fn commit_to_track_related_action(
    commit: &github_client::defs::Commit,
) -> Option<TrackRelatedAction> {
    // The committer must be GitHub Actions
    if commit.committer_name != LOG_COMMITTER_NAME {
        return None;
    }

    // The commit must be made after the backup job started to work
    if commit.datetime < *LOG_STARTED_AT {
        return None;
    }

    // The action target must be a track
    let action_type = parser::commit_message_to_track_related_action_type(&commit.message)?;

    // Construct the action
    let action = match action_type {
        TrackRelatedActionType::Addition => {
            let before_playlist = parser::parse_playlist_snapshot(&commit.files[0].before).ok()?;
            let after_playlist = parser::parse_playlist_snapshot(&commit.files[0].after).ok()?;
            let extra_track = util::identify_extra_track(&before_playlist, &after_playlist)?;
            TrackRelatedAction {
                datetime: commit.datetime,
                action_type,
                source_playlist_id: None,
                destination_playlist_id: Some(after_playlist.id),
                track: extra_track,
            }
        }
        TrackRelatedActionType::Removal => {
            let before_playlist = parser::parse_playlist_snapshot(&commit.files[0].before).ok()?;
            let after_playlist = parser::parse_playlist_snapshot(&commit.files[0].after).ok()?;
            let extra_track = util::identify_extra_track(&before_playlist, &after_playlist)?;
            TrackRelatedAction {
                datetime: commit.datetime,
                action_type,
                source_playlist_id: Some(before_playlist.id),
                destination_playlist_id: None,
                track: extra_track,
            }
        }
        TrackRelatedActionType::Transfer => {
            let (source_file, destination_file) = if commit.files[0].added_line_count > 0 {
                (&commit.files[1], &commit.files[0])
            } else {
                (&commit.files[0], &commit.files[1])
            };
            let before_source_playlist =
                parser::parse_playlist_snapshot(&source_file.before).ok()?;
            let before_destination_playlist =
                parser::parse_playlist_snapshot(&destination_file.before).ok()?;
            let after_destination_playlist =
                parser::parse_playlist_snapshot(&destination_file.after).ok()?;
            let extra_track = util::identify_extra_track(
                &before_destination_playlist,
                &after_destination_playlist,
            )?;
            TrackRelatedAction {
                datetime: commit.datetime,
                action_type,
                source_playlist_id: Some(before_source_playlist.id),
                destination_playlist_id: Some(after_destination_playlist.id),
                track: extra_track,
            }
        }
        TrackRelatedActionType::Modification => {
            let before_playlist = parser::parse_playlist_snapshot(&commit.files[0].before).ok()?;
            let after_playlist = parser::parse_playlist_snapshot(&commit.files[0].after).ok()?;
            let modified_track = util::identify_modified_track(&before_playlist, &after_playlist)?;
            TrackRelatedAction {
                datetime: commit.datetime,
                action_type,
                source_playlist_id: Some((&after_playlist.id).to_string()),
                destination_playlist_id: Some((&after_playlist.id).to_string()),
                track: modified_track,
            }
        }
    };

    return Some(action);
}
