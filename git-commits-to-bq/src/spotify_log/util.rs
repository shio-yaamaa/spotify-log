use std::collections::HashMap;

use crate::spotify_log::defs::{Playlist, Track};

pub fn identify_extra_track(playlist1: &Playlist, playlist2: &Playlist) -> Option<Track> {
  let playlist1_track_count = playlist1.tracks.len();
  let playlist2_track_count = playlist2.tracks.len();
  let (playlist_with_extra_track, playlist_without_extra_track) =
    if playlist1_track_count > playlist2_track_count {
      (playlist1, playlist2)
    } else {
      (playlist2, playlist1)
    };
  let mut track_id_to_track = HashMap::new();
  for track in &playlist_without_extra_track.tracks {
    track_id_to_track.insert(&track.id, track);
  }
  for track in &playlist_with_extra_track.tracks {
    if track_id_to_track.get(&track.id).is_none() {
      return Some(track.clone());
    }
  }
  return None;
}

pub fn identify_modified_track(playlist1: &Playlist, playlist2: &Playlist) -> Option<Track> {
  let mut track_id_to_track = HashMap::new();
  for track in &playlist1.tracks {
    track_id_to_track.insert(&track.id, track);
  }
  for track in &playlist2.tracks {
    if let Some(corresponding_track) = track_id_to_track.get(&track.id) {
      if !tracks_equal(&track, corresponding_track) {
        return Some(track.clone());
      }
    }
  }
  return None;
}

pub fn tracks_equal(track1: &Track, track2: &Track) -> bool {
  if track1.name != track2.name {
    return false;
  }
  if track1.artists.len() != track2.artists.len() {
    return false;
  }
  for (artist1, artist2) in track1.artists.iter().zip(track2.artists.iter()) {
    if artist1.id != artist2.id || artist1.name != artist2.name {
      return false;
    }
  }
  return true;
}
