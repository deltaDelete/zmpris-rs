use zbus_macros::proxy;

#[proxy(
    interface = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait MediaPlayer2 {
    /// Causes the media player to stop running.
    ///
    /// The media player may refuse to allow clients to shut it down.
    /// In this case, the [`can_quit`](Self::can_quit) property is
    /// **false** and this method does nothing.
    ///
    /// > Media players which can be D-Bus activated, or for which there is
    /// > no sensibly easy way to terminate a running instance (via the main
    /// > interface or a notification area icon for example) should allow clients
    /// > to use this method. Otherwise, it should not be needed.
    ///
    /// If the media player does not have a UI, this should be implemented.
    fn quit(&self) -> zbus::Result<()>;

    /// Brings the media player's user interface to the front using any
    /// appropriate mechanism available.
    ///
    /// The media player may be unable to control how its user interface
    /// is displayed, or it may not have a graphical user interface at all.
    /// In this case, the [`can_raise`](Self::can_raise) property is
    /// **false** and this method does nothing.
    fn raise(&self) -> zbus::Result<()>;

    /// If **false**, calling
    /// [`quit`](Self::quit) will have no effect, and may
    /// result in [`zbus::Error::Unsupported`].  If **true**, calling
    /// [`quit`](Self::quit) will cause the media application
    /// to attempt to quit (although it may still be prevented from quitting
    /// by the user, for example).
    #[zbus(property)]
    fn can_quit(&self) -> zbus::Result<bool>;

    /// If **false**, calling
    /// [`raise`](Self::raise) will have no effect, and may
    /// result in [`zbus::Error::Unsupported`].  If **true**, calling
    /// [`raise`](Self::raise) will cause the media application
    /// to attempt to bring its user interface to the front, although it may
    /// be prevented from doing so (by the window manager, for example).
    #[zbus(property)]
    fn can_raise(&self) -> zbus::Result<bool>;

    ///
    /// If **false**, attempting to set
    /// [`fullscreen`](Self::fullscreen) will have no effect, and may
    /// raise an error.  If **true**, attempting to set
    /// [`fullscreen`](Self::fullscreen) will not raise an error, and (if it
    /// is different from the current value) will cause the media player to attempt to
    /// enter or exit fullscreen mode.
    ///
    /// Note that the media player may be unable to fulfil the request.
    /// In this case, the value will not change.  If the media player knows in
    /// advance that it will not be able to fulfil the request, however, this
    /// property should be **false**.
    ///
    /// > This allows clients to choose whether to display controls for entering
    /// > or exiting fullscreen mode.
    #[zbus(property)]
    fn can_set_fullscreen(&self) -> zbus::Result<bool>;

    /// The basename of an installed .desktop file which complies with the [Desktop entry specification](http://standards.freedesktop.org/desktop-entry-spec/latest/),
    /// with the ".desktop" extension stripped.
    ///
    /// ##### Example
    /// The desktop entry file is "/usr/share/applications/vlc.desktop",
    /// and this property contains "vlc"
    #[zbus(property)]
    fn desktop_entry(&self) -> zbus::Result<String>;


    /// Whether the media player is occupying the fullscreen.
    ///
    /// This is typically used for videos.  A value of **true**
    /// indicates that the media player is taking up the full screen.
    ///
    /// Media centre software may well have this value fixed to **true**
    ///
    /// If [`can_set_fullscreen`](Self::can_set_fullscreen) is **true**,
    /// clients may set this property to **true** to tell the media player
    /// to enter fullscreen mode, or to **false** to return to windowed
    /// mode.
    ///
    /// If [`can_set_fullscreen`](Self::can_set_fullscreen) is **false**,
    /// then attempting to set this property should have no effect, and may raise
    /// an error.  However, even if it is **true**, the media player
    /// may still be unable to fulfil the request, in which case attempting to set
    /// this property will have no effect (but should not raise an error).
    ///
    /// > This allows remote control interfaces, such as LIRC or mobile devices like
    /// > phones, to control whether a video is shown in fullscreen.
    #[zbus(property)]
    fn fullscreen(&self) -> zbus::Result<bool>;

    /// See [`fullscreen`](Self::fullscreen).
    #[zbus(property)]
    fn set_fullscreen(&self, value: bool) -> zbus::Result<()>;

    /// Indicates whether the **/org/mpris/MediaPlayer2**
    /// object implements the **org.mpris.MediaPlayer2.TrackList**
    /// interface.
    #[zbus(property)]
    fn has_track_list(&self) -> zbus::Result<bool>;

    /// A friendly name to identify the media player to users.
    ///
    /// This should usually match the name found in .desktop files
    /// (eg: "VLC media player").
    #[zbus(property)]
    fn identity(&self) -> zbus::Result<String>;

    /// The mime-types supported by the media player.
    ///
    /// Mime-types should be in the standard format (eg: audio/mpeg or
    /// application/ogg).
    ///
    /// > This is important for clients to know when using the editing
    /// > capabilities of the Playlist interface, for example.
    #[zbus(property)]
    fn supported_mime_types(&self) -> zbus::Result<Vec<String>>;

    /// The URI schemes supported by the media player.
    ///
    /// This can be viewed as protocols supported by the player in almost
    /// all cases.  Almost every media player will include support for the
    /// "file" scheme.  Other common schemes are "http" and "rtsp".
    ///
    /// Note that URI schemes should be lower-case.
    ///
    /// > This is important for clients to know when using the editing
    /// > capabilities of the Playlist interface, for example.
    #[zbus(property)]
    fn supported_uri_schemes(&self) -> zbus::Result<Vec<String>>;
}
