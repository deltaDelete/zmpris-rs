use zvariant::ObjectPath;

/// Unique track identifier.
///
/// If the media player implements the TrackList interface and allows
/// the same track to appear multiple times in the tracklist,
/// this must be unique within the scope of the tracklist.
///
/// Note that this should be a valid D-Bus object id, although clients
/// should not assume that any object is actually exported with any
/// interfaces at that path.
///
/// Media players may not use any paths starting with
/// `/org/mpris` unless explicitly allowed by this specification.
/// Such paths are intended to have special meaning, such as
/// `/org/mpris/MediaPlayer2/TrackList/NoTrack`
/// to indicate "no track".
///
/// > This is a D-Bus object id as that is the definitive way to have
/// > unique identifiers on D-Bus.  It also allows for future optional
/// > expansions to the specification where tracks are exported to D-Bus
/// > with an interface similar to org.gnome.UPnP.MediaItem2.
pub type TrackId<'a> = ObjectPath<'a>;

/// Time in microseconds.
pub type TimeInUs = i64;

/// Audio volume level
///
/// - 0.0 means mute.
/// - 1.0 is a sensible maximum volume level (ex: 0dB).
///
/// Note that the volume may be higher than 1.0, although generally
/// clients should not attempt to set it above 1.0.
pub type Volume = f64;

/// A playback rate
///
/// This is a multiplier, so a value of 0.5 indicates that playback is
/// happening at half speed, while 1.5 means that 1.5 seconds of "track time"
/// is consumed every second.
pub type PlaybackRate = f64;

/// A unique resource identifier.
pub type Uri<'a> = &'a str;

pub trait TrackIdExt {
    fn is_no_track(&self) -> bool;
}

impl TrackIdExt for TrackId<'_> {
    fn is_no_track(&self) -> bool {
        self.eq("/org/mpris/MediaPlayer2/TrackList/NoTrack")
    }
}