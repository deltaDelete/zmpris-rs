
use zbus::proxy;
use crate::shared::TrackId;

/// TODO: DOCS
#[proxy(
    interface = "org.mpris.MediaPlayer2.TrackList",
    default_service = "org.mpris.MediaPlayer2.playerctld",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait TrackList {
    /// AddTrack method
    fn add_track(
        &self,
        uri: &str,
        after_track: &TrackId<'_>,
        set_as_current: bool,
    ) -> zbus::Result<()>;

    /// GetTracksMetadata method
    fn get_tracks_metadata(
        &self,
        track_ids: &[&TrackId<'_>],
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// GoTo method
    fn go_to(&self, track_id: &TrackId<'_>) -> zbus::Result<()>;

    /// RemoveTrack method
    fn remove_track(&self, track_id: &TrackId<'_>) -> zbus::Result<()>;

    /// TrackAdded signal
    #[zbus(signal)]
    fn track_added(
        &self,
        metadata: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        after_track: TrackId<'_>,
    ) -> zbus::Result<()>;

    /// TrackListReplaced signal
    #[zbus(signal)]
    fn track_list_replaced(
        &self,
        tracks: Vec<TrackId<'_>>,
        current_track: TrackId<'_>,
    ) -> zbus::Result<()>;

    /// TrackMetadataChanged signal
    #[zbus(signal)]
    fn track_metadata_changed(
        &self,
        track_id: TrackId<'_>,
        metadata: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// TrackRemoved signal
    #[zbus(signal)]
    fn track_removed(&self, track_id: TrackId<'_>) -> zbus::Result<()>;

    /// CanEditTracks property
    #[zbus(property)]
    fn can_edit_tracks(&self) -> zbus::Result<bool>;

    /// Tracks property
    #[zbus(property(emits_changed_signal = "invalidates"))]
    fn tracks(&self) -> zbus::Result<Vec<TrackId>>;
}
