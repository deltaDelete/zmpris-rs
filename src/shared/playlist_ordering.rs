use crate::shared::W;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;
use zvariant::serialized::Context;
use zvariant::{OwnedValue, Structure, Type, Value, NATIVE_ENDIAN};

/// Specifies the ordering of returned playlists.
#[derive(Deserialize, Serialize, Type, PartialEq, Eq, Debug, Hash, Copy, Clone)]
#[zvariant(signature = "s")]
pub enum PlaylistOrdering {
    /// Alphabetical ordering by name, ascending.
    Alphabetical,
    /// Ordering by creation date, oldest first.
    Created,
    /// Ordering by last modified date, oldest first.
    Modified,
    /// Ordering by date of last playback, oldest first.
    Played,
    /// A user-defined ordering.
    /// > Some media players may allow users to order playlists as they
    /// > wish. This ordering allows playlists to be retrieved in that
    /// > order.
    User
}


impl FromStr for PlaylistOrdering {
    type Err = core::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(From::from(s))
    }
}

impl From<&str> for PlaylistOrdering {
    fn from(value: &str) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, value).expect("Failed to serialize string to bytes");
        result.deserialize()
            .map(|s| s.0)
            .expect("Failed to deserialize bytes to PlaybackStatus")
    }
}

impl From<PlaylistOrdering> for String {
    fn from(value: PlaylistOrdering) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, &value).expect("Failed to serialize PlaybackStatus to bytes");
        result.deserialize().map(|s| s.0).expect("Failed to deserialize bytes to String")
    }
}

impl TryFrom<OwnedValue> for PlaylistOrdering {
    type Error = zbus::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        match value.deref() {
            Value::Str(value) => Ok(From::from(value.as_str())),
            _ => Err(zbus::Error::Unsupported),
        }
    }
}

impl<'a> TryFrom<&Value<'a>> for PlaylistOrdering {
    type Error = zbus::Error;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Str(value) => Ok(From::from(value.as_str())),
            _ => Err(zbus::Error::Unsupported),
        }
    }
}

impl<'a> From<PlaylistOrdering> for Structure<'a> {
    fn from(value: PlaylistOrdering) -> Self {
        Structure::from((String::from(value),))
    }
}

impl TryFrom<OwnedValue> for W<Vec<PlaylistOrdering>> {
    type Error = zbus::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        match value.deref() {
            Value::Array(array) => {
                Ok(W(array.iter().flat_map(PlaylistOrdering::try_from).collect()))
            }
            _ => Err(zbus::Error::Unsupported)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::PlaylistOrdering;
    use test_log::test;
    use zvariant::Value;

    #[test(tokio::test)]
    async fn enum_conversion() -> anyhow::Result<()> {
        let items = std::collections::HashMap::from([
            (PlaylistOrdering::Alphabetical, "Alphabetical"),
            (PlaylistOrdering::Created, "Created"),
            (PlaylistOrdering::Modified, "Modified"),
            (PlaylistOrdering::Played, "Played"),
            (PlaylistOrdering::User, "User"),
        ]);

        for (status, string) in items {
            assert_eq!(String::from(status), string);
            assert_eq!(status, PlaylistOrdering::from(string));
        }

        anyhow::Ok(())
    }

    #[test(tokio::test)]
    async fn from_owned_value() -> anyhow::Result<()> {
        let value = Value::from("User").try_to_owned()?;

        let result = PlaylistOrdering::try_from(value)?;
        assert_eq!(result, PlaylistOrdering::User);

        anyhow::Ok(())
    }
}