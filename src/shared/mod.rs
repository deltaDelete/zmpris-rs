mod loop_status;
mod playback_status;
mod playlist_ordering;
mod playlist_struct;
mod type_alias;

use std::fmt::Display;
use std::ops::Deref;
pub use loop_status::*;
pub use playback_status::*;
pub use playlist_ordering::*;
pub use playlist_struct::*;
pub use type_alias::*;

pub const BASE_PATH: &str = "org.mpris.MediaPlayer2.";

/// Wrapper struct that allows to implement traits using foreign types
#[derive(Debug, Clone, Copy)]
pub struct W<T>(pub T);

impl<T> Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Display for W<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}