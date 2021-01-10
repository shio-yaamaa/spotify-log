use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    // Ignore addedAt property
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub enum TrackRelatedActionType {
    Addition,
    Removal,
    Transfer,
    Modification,
}

#[derive(Debug)]
pub struct TrackRelatedAction {
    pub datetime: DateTime<Utc>,
    pub action_type: TrackRelatedActionType,
    pub source_playlist_id: Option<String>,
    pub destination_playlist_id: Option<String>,
    pub track: Track,
}
