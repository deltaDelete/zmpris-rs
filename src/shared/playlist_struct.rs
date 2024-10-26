use serde::{Deserialize, Serialize};
use zvariant::{OwnedObjectPath, OwnedValue, Type, Value};
use crate::shared::W;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Value, OwnedValue)]
pub struct Playlist {
    playlist_id: OwnedObjectPath,
    name: String,
    uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Value, OwnedValue)]
struct MaybePlaylist {
    has_value: bool,
    playlist: Playlist,
}

impl TryFrom<OwnedValue> for W<Option<Playlist>> {
    type Error = zbus::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let playlist = MaybePlaylist::try_from(value).map_err(zbus::Error::Variant)?;
        if playlist.has_value {
            Ok(W(Some(playlist.playlist.clone())))
        } else {
            Ok(W(None))
        }
    }
}
