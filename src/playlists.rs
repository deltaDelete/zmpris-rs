use crate::shared::{Playlist, PlaylistOrdering, W};
use zbus::proxy;
use zvariant::ObjectPath;

/// Provides access to the media player's playlists.
///
/// Since D-Bus does not provide an easy way to check for what interfaces
/// are exported on an object, clients should attempt to get one of the
/// properties on this interface to see if it is implemented.
#[proxy(
    interface = "org.mpris.MediaPlayer2.Playlists",
    default_path = "/org/mpris/MediaPlayer2"
)]
trait Playlists {
    /// Starts playing the given playlist.
    ///
    /// Note that this must be implemented.  If the media player does not
    /// allow clients to change the playlist, it should not implement this
    /// interface at all.
    ///
    /// It is up to the media player whether this completely replaces the
    /// current tracklist, or whether it is merely inserted into the
    /// tracklist and the first track starts.  For example, if the media
    /// player is operating in a "jukebox" mode, it may just append the
    /// playlist to the list of upcoming tracks, and skip to the first
    /// track in the playlist.
    ///
    /// ## Parameters
    /// - `playlist_id`: the id of the playlist to activate.
    fn activate_playlist(&self, playlist_id: &ObjectPath<'_>) -> zbus::Result<()>;

    /// Gets a set of playlists.
    ///
    /// ## Parameters
    /// - `index`: the index of the first playlist to be fetched (according to the ordering).
    /// - `max_count`: the maximum number of playlists to fetch.
    /// - `order`: the ordering that should be used.
    /// - `reverse_order`: whether the order should be reversed.
    ///
    /// ## Returns
    /// A list of (at most MaxCount) playlists.
    fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> zbus::Result<Vec<Playlist>>;

    /// Indicates that either the Name or Icon attribute of a
    /// playlist has changed.
    ///
    /// Client implementations should be aware that this signal
    /// may not be implemented.
    ///
    /// > Without this signal, media players have no way to notify clients
    /// > of a change in the attributes of a playlist other than the active one
    ///
    /// ## Parameters:
    /// - `playlist`: the playlist which details have changed.
    #[zbus(signal)]
    fn playlist_changed(
        &self,
        playlist: Playlist,
    ) -> zbus::Result<()>;

    /// The currently-active playlist.
    ///
    /// If there is no currently-active playlist, the structure's Valid field
    /// will be false, and the Playlist details are undefined.
    ///
    /// Note that this may not have a value even after ActivatePlaylist is
    /// called with a valid playlist id as ActivatePlaylist implementations
    /// have the option of simply inserting the contents of the playlist into
    /// the current tracklist.
    #[zbus(property)]
    fn active_playlist(
        &self,
    ) -> zbus::Result<W<Option<Playlist>>>;

    /// The available orderings.  At least one must be offered.
    ///
    /// > Media players may not have access to all the data required for some
    /// > orderings.  For example, creation times are not available on UNIX
    /// > filesystems (don't let the ctime fool you!).  On the other hand,
    /// > clients should have some way to get the "most recent" playlists.
    #[zbus(property)]
    fn orderings(&self) -> zbus::Result<W<Vec<PlaylistOrdering>>>;

    /// The number of playlists available.
    #[zbus(property)]
    fn playlist_count(&self) -> zbus::Result<u32>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::discovery::by_name;
    use anyhow::Result;
    use log::info;
    use test_log::test;
    use zbus::Connection;

    #[test(tokio::test)]
    #[ignore]
    async fn test_get_playlists() -> Result<()> {
        let conn = Connection::session().await?;
        let proxy: PlaylistsProxy = by_name(&conn, "org.mpris.MediaPlayer2.org.gnome.Music").await?;

        let result = proxy.get_playlists(0, 100, PlaylistOrdering::Alphabetical, false).await?;
        info!("{:?}", result);

        let orderings = proxy.orderings().await?;
        info!("{:?}", orderings);

        let active_playlist = proxy.active_playlist().await?;
        info!("{:?}", active_playlist);

        Ok(())
    }
}