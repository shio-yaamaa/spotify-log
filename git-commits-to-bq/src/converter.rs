use serde::Serialize;
use std::collections::HashMap;

use crate::spotify_log::defs::{Artist, Track, TrackRelatedAction, TrackRelatedActionType};

#[derive(Debug, Serialize)]
pub struct ActionTableRow {
    pub timestamp: String,
    pub action_type: String,
    pub source_playlist_id: Option<String>,
    pub destination_playlist_id: Option<String>,
    pub track_id: String,
}

#[derive(Debug, Serialize)]
pub struct TrackTableRow {
    pub id: String,
    pub name: String,
    pub artist_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ArtistTableRow {
    pub id: String,
    pub name: String,
}

pub fn track_related_action_to_table_rows(
    actions: Vec<TrackRelatedAction>,
) -> (Vec<ActionTableRow>, Vec<TrackTableRow>, Vec<ArtistTableRow>) {
    let mut track_id_to_track_row = HashMap::new();
    let mut artist_id_to_artist_row = HashMap::new();

    let mut action_rows = vec![];

    for action in actions {
        match action.action_type {
            TrackRelatedActionType::Addition => {
                action_rows.push(ActionTableRow {
                    timestamp: action.datetime.to_rfc3339(),
                    action_type: "addition".to_string(),
                    source_playlist_id: action.source_playlist_id.clone(),
                    destination_playlist_id: action.destination_playlist_id.clone(),
                    track_id: action.track.id.to_string(),
                });
                track_id_to_track_row.insert(
                    action.track.id.to_string(),
                    track_to_track_table_row(&action.track),
                );
                for artist in action.track.artists {
                    artist_id_to_artist_row
                        .insert(artist.id.to_string(), artist_to_artist_table_row(&artist));
                }
            }
            TrackRelatedActionType::Removal => {
                action_rows.push(ActionTableRow {
                    timestamp: action.datetime.to_rfc3339(),
                    action_type: "removal".to_string(),
                    source_playlist_id: action.source_playlist_id.clone(),
                    destination_playlist_id: action.destination_playlist_id.clone(),
                    track_id: action.track.id.to_string(),
                });
            }
            TrackRelatedActionType::Transfer => {
                action_rows.push(ActionTableRow {
                    timestamp: action.datetime.to_rfc3339(),
                    action_type: "transfer".to_string(),
                    source_playlist_id: action.source_playlist_id.clone(),
                    destination_playlist_id: action.destination_playlist_id.clone(),
                    track_id: action.track.id.to_string(),
                });
            }
            TrackRelatedActionType::Modification => {
                track_id_to_track_row.insert(
                    action.track.id.to_string(),
                    track_to_track_table_row(&action.track),
                );
                for artist in action.track.artists {
                    artist_id_to_artist_row
                        .insert(artist.id.to_string(), artist_to_artist_table_row(&artist));
                }
            }
        }
    }

    let track_rows = track_id_to_track_row
        .into_iter()
        .map(|(_id, track)| track)
        .collect();
    let artist_rows = artist_id_to_artist_row
        .into_iter()
        .map(|(_id, artist)| artist)
        .collect();

    return (action_rows, track_rows, artist_rows);
}

fn track_to_track_table_row(track: &Track) -> TrackTableRow {
    return TrackTableRow {
        id: track.id.to_string(),
        name: track.name.to_string(),
        artist_ids: track
            .artists
            .iter()
            .map(|artist| artist.id.to_string())
            .collect(),
    };
}

fn artist_to_artist_table_row(artist: &Artist) -> ArtistTableRow {
    return ArtistTableRow {
        id: artist.id.to_string(),
        name: artist.name.to_string(),
    };
}
