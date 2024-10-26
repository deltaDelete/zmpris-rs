use std::ops::Deref;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Structure, Type, Value, NATIVE_ENDIAN};
use zvariant::serialized::Context;

/// A repeat / loop status
#[derive(Deserialize, Serialize, Type, PartialEq, Eq, Debug, Hash, Copy, Clone)]
#[zvariant(signature = "s")]
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,
    /// The current track will start again from the begining once it has finished playing
    Track,
    /// The playback loops through a list of tracks
    Playlist,
}

impl FromStr for LoopStatus {
    type Err = core::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(From::from(s))
    }
}

impl From<LoopStatus> for String {
    fn from(value: LoopStatus) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, &value).expect("Failed to serialize LoopStatus to bytes");
        result.deserialize::<String>().map(|s| s.0).expect("Failed to deserialize bytes to string")
    }
}

impl From<&str> for LoopStatus {
    fn from(value: &str) -> Self {
        let ctx = Context::new_dbus(NATIVE_ENDIAN, 0);
        let result = zvariant::to_bytes(ctx, &value).expect("Failed to serialize string to bytes");
        result.deserialize::<LoopStatus>().map(|s| s.0).expect("Failed to deserialize bytes to LoopStatus")
    }
}

impl TryFrom<OwnedValue> for LoopStatus {
    type Error = zbus::Error;
    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        match value.deref() {
            Value::Str(value) => Ok(From::from(value.as_str())),
            _ => Err(zbus::Error::Unsupported),
        }
    }
}

impl<'a> From<LoopStatus> for Structure<'a> {
    fn from(value: LoopStatus) -> Self {
        Structure::from((String::from(value),))
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::LoopStatus;
    use test_log::test;
    use zvariant::Value;

    #[test(tokio::test)]
    async fn enum_conversion() -> anyhow::Result<()> {
        let items = std::collections::HashMap::from([
            (LoopStatus::None, "None"),
            (LoopStatus::Track, "Track"),
            (LoopStatus::Playlist, "Playlist"),
        ]);

        for (status, string) in items {
            assert_eq!(String::from(status), string);
            assert_eq!(status, LoopStatus::from(string));
        }

        anyhow::Ok(())
    }

    #[test(tokio::test)]
    async fn from_owned_value() -> anyhow::Result<()> {
        let value = Value::from("None").try_to_owned()?;

        let result = LoopStatus::try_from(value)?;
        assert_eq!(result, LoopStatus::None);

        anyhow::Ok(())
    }
}