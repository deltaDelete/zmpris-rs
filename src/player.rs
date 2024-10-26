use zbus::proxy;
use crate::shared::{LoopStatus, PlaybackRate, PlaybackStatus, TimeInUs, TrackId, Uri, Volume};

/// This interface implements the methods for querying and providing basic
///  control over what is currently playing.
#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Player {
    /// Skips to the next track in the tracklist.
    ///
    /// If there is no next track (and endless playback and track
    /// repeat are both off), stop playback.
    ///
    /// If playback is paused or stopped, it remains that way.
    ///
    /// If [`can_go_next`](Self::can_go_next) is
    /// **false**, attempting to call this method should have
    /// no effect.
    fn next(&self) -> zbus::Result<()>;

    /// kips to the previous track in the tracklist.
    ///
    /// If there is no previous track (and endless playback and track
    /// repeat are both off), stop playback.
    ///
    /// If playback is paused or stopped, it remains that way.</p>
    ///
    /// If [`can_go_previous`](Self::can_go_previous) is
    /// **false**, attempting to call this method should have
    /// no effect.
    fn previous(&self) -> zbus::Result<()>;

    /// Pauses playback.

    /// If playback is already paused, this has no effect.
    ///
    ///   Calling Play after this should cause playback to start again
    ///   from the same position.
    ///
    /// If [`can_pause`](Self::can_pause) is
    /// **false**, attempting to call this method should have
    /// no effect.
    fn pause(&self) -> zbus::Result<()>;

    /// Pauses playback.
    ///
    /// If playback is already paused, resumes playback.
    ///
    /// If playback is stopped, starts playback.
    ///
    /// If [`can_pause`](Self::can_pause) is
    /// **false**, attempting to call this method should have
    /// no effect and raise an error.
    fn play_pause(&self) -> zbus::Result<()>;

    /// Stops playback.
    ///
    /// If playback is already stopped, this has no effect.
    ///
    /// Calling Play after this should cause playback to
    /// start again from the beginning of the track.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, attempting to call this method should have
    /// no effect and raise an error.
    fn stop(&self) -> zbus::Result<()>;

    /// Starts or resumes playback.
    ///
    /// If already playing, this has no effect.
    ///
    /// If paused, playback resumes from the current position.
    ///
    /// If there is no track to play, this has no effect.
    ///
    /// If [`can_play`](Self::can_play) is
    /// **false**, attempting to call this method should have
    /// no effect.
    fn play(&self) -> zbus::Result<()>;

    /// Seeks forward in the current track by the specified number
    /// of microseconds.
    ///
    /// A negative value seeks back. If this would mean seeking
    /// back further than the start of the track, the position
    /// is set to 0.
    ///
    /// If the value passed in would mean seeking beyond the end
    /// of the track, acts like a call to Next.
    ///
    /// If the [`can_seek`](Self::can_seek) property is false,
    /// this has no effect.
    ///
    /// ## Parameters
    /// - `offset`: the number of microseconds to seek forward.
    fn seek(&self, offset: TimeInUs) -> zbus::Result<()>;

    /// Sets the current track position in microseconds.
    ///
    /// If the Position argument is less than 0, do nothing.
    ///
    /// If the Position argument is greater than the track length,
    /// do nothing.
    ///
    /// If the [`can_seek`](Self::can_seek) property is false,
    /// this has no effect.
    ///
    /// > The reason for having this method, rather than making
    /// > [`position`](Self::position) writable, is to include
    /// > the TrackId argument to avoid race conditions where a client tries
    /// > to seek to a position when the track has already changed.
    ///
    /// ## Parameters
    /// - TrackId: can be retrieved from metadata
    /// - Offset: track position in microseconds, between 0 and `track_length`
    fn set_position(
        &self,
        track_id: &TrackId<'_>,
        offset: TimeInUs,
    ) -> zbus::Result<()>;

    /// Opens the Uri given as an argument
    ///
    /// If the playback is stopped, starts playing
    ///
    /// If the uri scheme or the mime-type of the uri to open is not supported,
    /// this method does nothing and may raise an error.  In particular, if the
    /// list of available uri schemes is empty, this method may not be
    /// implemented.
    ///
    /// Clients should not assume that the Uri has been opened as soon as this
    /// method returns. They should wait until the mpris:trackid field in the
    /// [`metadata`](Self::metadata) property changes.
    ///
    /// If the media player implements the TrackList interface, then the
    /// opened track should be made part of the tracklist, the
    /// `org.mpris.MediaPlayer2.TrackList.TrackAdded` or
    /// `org.mpris.MediaPlayer2.TrackList.TrackListReplaced`
    /// signal should be fired, as well as the
    /// `org.freedesktop.DBus.Properties.PropertiesChanged`
    /// signal on the tracklist interface.
    ///
    /// ## Parameters
    /// - uri: Uri of the track to load. Its uri scheme should be an element of the
    /// [MediaPlayer2::supported_uri_schemes](crate::media_player::MediaPlayer2Proxy::supported_uri_schemes)
    /// property and the mime-type should match one of the elements of the
    /// [MediaPlayer2::supported_mime_types](crate::media_player::MediaPlayer2Proxy::supported_mime_types).
    fn open_uri(&self, uri: Uri<'_>) -> zbus::Result<()>;

    /// The current playback status.
    ///
    /// May be "Playing", "Paused" or "Stopped".
    ///
    /// Returns [PlaybackStatus]
    #[zbus(property)]
    fn playback_status(&self) -> zbus::Result<PlaybackStatus>;

    /// The current loop / repeat status
    /// May be:
    /// - "None" if the playback will stop when there are no more tracks to play
    /// - "Track" if the current track will start again from the begining once it has finished playing
    /// - "Playlist" if the playback loops through a list of tracks
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, attempting to set this property should have
    /// no effect and raise an error.
    ///
    /// Returns [LoopStatus](LoopStatus)
    #[zbus(property)]
    fn loop_status(&self) -> zbus::Result<LoopStatus>;
    /// Set [`loop_status`](Self::loop_status) property
    #[zbus(property)]
    fn set_loop_status(&self, value: LoopStatus) -> zbus::Result<()>;

    /// The current playback rate.
    ///
    /// The value must fall in the range described by
    /// [`minimum_rate`](Self::minimum_rate) and
    /// [`maximum_rate`](Self::maximum_rate), and must not be 0.0.  If
    /// playback is paused, the [`playback_status`](Self::playback_status)
    /// property should be used to indicate this.  A value of 0.0 should not
    /// be set by the client.  If it is, the media player should act as
    /// though [`pause`](Self::pause) was called.
    ///
    /// If the media player has no ability to play at speeds other than the
    /// normal playback rate, this must still be implemented, and must
    /// return 1.0.  The [`minimum_rate`](Self::minimum_rate) and
    /// [`maximum_rate`](Self::maximum_rate) properties must also be
    /// set to 1.0.
    ///
    /// Not all values may be accepted by the media player.  It is left to
    /// media player implementations to decide how to deal with values they
    /// cannot use; they may either ignore them or pick a "best fit" value.
    /// Clients are recommended to only use sensible fractions or multiples
    /// of 1 (eg: 0.5, 0.25, 1.5, 2.0, etc).
    ///
    /// > This allows clients to display (reasonably) accurate progress bars
    /// > without having to regularly query the media player for the current
    /// > position.
    #[zbus(property)]
    fn rate(&self) -> zbus::Result<PlaybackRate>;

    /// See [`rate`](Self::rate).
    #[zbus(property)]
    fn set_rate(&self, value: PlaybackRate) -> zbus::Result<()>;

    /// A value of **false** indicates that playback is
    /// progressing linearly through a playlist, while **true**
    /// means playback is progressing through a playlist in some other order.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, attempting to set this property should have
    /// no effect and raise an error.
    #[zbus(property)]
    fn shuffle(&self) -> zbus::Result<bool>;

    /// See [`shuffle`](Self::shuffle).
    #[zbus(property)]
    fn set_shuffle(&self, value: bool) -> zbus::Result<()>;

    /// The metadata of the current element.
    ///
    /// If there is a current track, this must have a "mpris:trackid" entry
    /// (of D-Bus type "o") at the very least, which contains a D-Bus path that
    /// uniquely identifies this track.
    ///
    /// See the type documentation for more details.
    // TODO: Replace generic hashmap with more specialized structure
    #[zbus(property)]
    fn metadata(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::Value>>;

    /// he volume level.
    ///
    /// When setting, if a negative value is passed, the volume
    /// should be set to 0.0.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, attempting to set this property should have
    /// no effect and raise an error.
    #[zbus(property)]
    fn volume(&self) -> zbus::Result<Volume>;

    /// See [`volume`](Self::volume).
    #[zbus(property)]
    fn set_volume(&self, value: Volume) -> zbus::Result<()>;

    /// The current track position in microseconds, between 0 and
    /// the 'mpris:length' metadata entry (see Metadata).
    ///
    /// Note: If the media player allows it, the current playback position
    /// can be changed either the SetPosition method or the Seek method on
    /// this interface.  If this is not the case, the
    /// [`can_seek`](Self::can_seek) property is false, and
    /// setting this property has no effect and can raise an error.
    ///
    /// If the playback progresses in a way that is inconstistant with the
    /// [`rate`](Self::rate) property, the
    /// [`seeked`](Self::seeked) signal is emited.
    #[zbus(property(emits_changed_signal = "false"))]
    fn position(&self) -> zbus::Result<TimeInUs>;

    /// The minimum value which the [`rate`](Self::rate)
    /// property can take.
    /// Clients should not attempt to set the
    /// [`rate`](Self::rate) property below this value.
    ///
    /// Note that even if this value is 0.0 or negative, clients should
    /// not attempt to set the [`rate`](Self::rate) property
    /// to 0.0.
    ///
    /// This value should always be 1.0 or less.
    #[zbus(property)]
    fn minimum_rate(&self) -> zbus::Result<PlaybackRate>;

    /// The maximum value which the [`rate`](Self::rate)
    /// property can take.
    /// Clients should not attempt to set the
    /// [`rate`](Self::rate) property above this value.
    ///
    /// This value should always be 1.0 or greater.
    #[zbus(property)]
    fn maximum_rate(&self) -> zbus::Result<PlaybackRate>;

    /// Whether the client can call the [`next`](Self::next)
    /// method on this interface and expect the current track to change.
    ///
    /// If it is unknown whether a call to [`next`](Self::next) will
    /// be successful (for example, when streaming tracks), this property should
    /// be set to **true**.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, this property should also be
    /// **false**.
    ///
    /// > Even when playback can generally be controlled, there may not
    /// > always be a next track to move to.
    #[zbus(property)]
    fn can_go_next(&self) -> zbus::Result<bool>;

    /// Whether the client can call the
    /// [`previous`](Self::previous) method on this interface and
    /// expect the current track to change.
    ///
    /// If it is unknown whether a call to [`previous`](Self::previous)
    /// will be successful (for example, when streaming tracks), this property
    /// should be set to **true**.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, this property should also be
    /// **false**.
    ///
    /// > Even when playback can generally be controlled, there may not
    /// > always be a next previous to move to.
    #[zbus(property)]
    fn can_go_previous(&self) -> zbus::Result<bool>;

    /// Whether playback can be started using
    /// [`play`](Self::play) or
    /// [`play_pause`](Self::play_pause).
    ///
    /// Note that this is related to whether there is a "current track": the
    /// value should not depend on whether the track is currently paused or
    /// playing.  In fact, if a track is currently playing (and
    /// [`can_control`](Self::can_control) is **true**),
    /// this should be **true**.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, this property should also be
    /// **false**.
    ///
    /// > Even when playback can generally be controlled, it may not be
    /// > possible to enter a "playing" state, for example if there is no
    /// > "current track".
    #[zbus(property)]
    fn can_play(&self) -> zbus::Result<bool>;

    /// <p>Whether playback can be paused using
    /// [`pause`](Self::pause) or
    /// [`play_pause`](Self::play_pause).
    ///
    /// Note that this is an intrinsic property of the current track: its
    /// value should not depend on whether the track is currently paused or
    /// playing.  In fact, if playback is currently paused (and
    /// [`can_control`](Self::can_control) is **true**),
    /// this should be **true**.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, this property should also be
    /// **false**.
    ///
    /// > Not all media is pausable: it may not be possible to pause some
    /// > streamed media, for example.
    #[zbus(property)]
    fn can_pause(&self) -> zbus::Result<bool>;

    /// Whether the client can control the playback position using
    /// [`seek`](Self::seek) and
    /// [`set_position`](Self::set_position).  This may be different for
    /// different tracks.
    ///
    /// If [`can_control`](Self::can_control) is
    /// **false**, this property should also be
    /// **false**.
    ///
    /// > Not all media is seekable: it may not be possible to seek when
    /// > playing some streamed media, for example.
    #[zbus(property)]
    fn can_seek(&self) -> zbus::Result<bool>;

    /// Whether the media player may be controlled over this interface.
    ///
    /// This property is not expected to change, as it describes an intrinsic
    /// capability of the implementation.
    ///
    /// If this is **false**, clients should assume that all
    /// properties on this interface are read-only (and will raise errors
    /// if writing to them is attempted), no methods are implemented
    /// and all other properties starting with "Can" are also
    /// **false**.
    ///
    /// > This allows clients to determine whether to present and enable
    /// > controls to the user in advance of attempting to call methods
    /// > and write to properties.
    #[zbus(property(emits_changed_signal = "false"))]
    fn can_control(&self) -> zbus::Result<bool>;

    /// Indicates that the track position has changed in a way that is
    /// inconsistant with the current playing state.
    ///
    /// When this signal is not received, clients should assume that:
    /// - When playing, the position progresses according to the rate property.
    /// - When paused, it remains constant.
    ///
    /// This signal does not need to be emitted when playback starts
    /// or when the track changes, unless the track is starting at an
    /// unexpected position. An expected position would be the last
    /// known one when going from Paused to Playing, and 0 when going from
    /// Stopped to Playing.
    ///
    /// ## Parameters
    /// - `position`: the new position, in microseconds.
    #[zbus(signal)]
    fn seeked(&self, position: TimeInUs) -> zbus::Result<()>;
}

#[cfg(test)]
mod test {
    use crate::sync::{MediaPlayer2Proxy, PlayerProxy};
    use anyhow::{bail, Ok};
    use anyhow::{anyhow, Result};
    use futures::StreamExt;
    use log::info;
    use std::time::Duration;
    use test_log::test;
    use zbus::Connection;
    use zvariant::{ObjectPath, Value};
    use crate::sync::discovery::{all, by_name, currently_playing, first};

    #[test(tokio::test)]
    async fn get_all_players() -> Result<()> {
        let conn: Connection = Connection::session().await?;
        let proxies: Vec<MediaPlayer2Proxy> = all(&conn).await?;

        for proxy in proxies {
            let identity = proxy.identity().await?.clone();
            info!("Found proxy: {:?}", identity);
        }

        Ok(())
    }

    #[test(tokio::test)]
    async fn toggle_first() -> Result<()> {
        let conn = Connection::session().await?;
        let player: MediaPlayer2Proxy = first(&conn).await?;

        info!("Got player: {}", player.inner().destination().to_string());
        let player = PlayerProxy::new(&conn, player.inner().destination().to_string()).await?;
        info!("{:?}", player.play_pause().await?);

        Ok(())
    }

    #[test(tokio::test)]
    #[ignore]
    async fn stream_players() -> Result<()> {
        let conn = Connection::session().await?;
        let proxy: PlayerProxy = currently_playing(&conn).await?;

        let mut playback_status_stream = proxy.receive_playback_status_changed().await;
        let msg = playback_status_stream.next().await.ok_or(anyhow!("No change"))?;
        let msg = msg.get().await?;
        info!("Got message: {:?}", msg);

        Ok(())
    }

    #[test(tokio::test)]
    #[ignore]
    async fn position() -> Result<()> {
        let conn = Connection::session().await?;
        let proxy: PlayerProxy = by_name(&conn, "org.mpris.MediaPlayer2.spotify").await?;

        let mut interval = tokio::time::interval(Duration::from_millis(500));
        for _ in 0..10 {
            interval.tick().await;
            let i = proxy.position().await?;
            info!("Got position: {:?}", Duration::from_micros(i as u64));
        }

        Ok(())
    }

    #[test(tokio::test)]
    #[ignore]
    async fn test_set_position() -> Result<()> {
        let conn = Connection::session().await?;
        let proxy: PlayerProxy = currently_playing(&conn).await?;
        let metadata = proxy.metadata().await?;
        let track_id = metadata.get("mpris:trackid").ok_or(anyhow!("No mpris:trackid"))?;
        info!("Track id: {:?}\nType: {:?}", track_id, track_id.value_signature());
        let Value::Str(track_id) = track_id else {
            bail!("");
        };
        let object_path = ObjectPath::try_from(track_id.to_string())?;

        proxy.set_position(&object_path, 10*1000000).await?;

        Ok(())
    }
}
