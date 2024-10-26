use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;
use zvariant::serialized::Context;
use zvariant::{OwnedValue, Type, Value, NATIVE_ENDIAN};

/// A playback state
#[derive(Deserialize, Serialize, Type, PartialEq, Eq, Debug, Hash, Copy, Clone)]
#[zvariant(signature = "s")]
pub enum PlaybackStatus {
    /// A track is currently playing
    Playing,
    /// A track is currently paused
    Paused,
    /// There is no track currently playing
    Stopped,
}

impl FromStr for PlaybackStatus {
    type Err = core::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(From::from(s))
    }
}

impl From<&str> for PlaybackStatus {
    fn from(value: &str) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, value).expect("Failed to serialize string to bytes");
        result.deserialize()
            .map(|s| s.0)
            .expect("Failed to deserialize bytes to PlaybackStatus")
    }
}

impl From<PlaybackStatus> for String {
    fn from(value: PlaybackStatus) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, &value).expect("Failed to serialize PlaybackStatus to bytes");
        result.deserialize().map(|s| s.0).expect("Failed to deserialize bytes to String")
    }
}

impl TryFrom<OwnedValue> for PlaybackStatus {
    type Error = zbus::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        match value.deref() {
            Value::Str(value) => Ok(From::from(value.as_str())),
            _ => Err(zbus::Error::Unsupported),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::PlaybackStatus;
    use test_log::test;
    use zvariant::Value;

    #[test(tokio::test)]
    async fn enum_conversion() -> anyhow::Result<()> {
        let items = std::collections::HashMap::from([
            (PlaybackStatus::Paused, "Paused"),
            (PlaybackStatus::Stopped, "Stopped"),
            (PlaybackStatus::Playing, "Playing"),
        ]);

        for (status, string) in items {
            assert_eq!(String::from(status), string);
            assert_eq!(status, PlaybackStatus::from(string));
        }

        anyhow::Ok(())
    }

    #[test(tokio::test)]
    async fn from_owned_value() -> anyhow::Result<()> {
        let value = Value::from("Paused").try_to_owned()?;

        let result = PlaybackStatus::try_from(value)?;
        assert_eq!(result, PlaybackStatus::Paused);

        anyhow::Ok(())
    }
}