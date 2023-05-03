use core::fmt;
use steamstacks_bindings as bindings;

pub use crate::apps::*;
pub use crate::friends::*;
pub use crate::user::*;
pub use crate::utils::*;

pub mod apps;
pub mod friends;
pub mod steam_api;
pub mod user;
pub mod utils;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

/// An id for a steam app/game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AppId(pub u32);

impl From<u32> for AppId {
    fn from(id: u32) -> Self {
        AppId(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SteamId(pub(crate) u64);

impl fmt::Display for SteamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type SResult<T> = Result<T, SteamResult>;
