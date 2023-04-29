use core::fmt;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;
use steamstacks_bindings as bindings;
use utils::error::SteamError;

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

pub(crate) unsafe trait Manager {
    fn get_pipe() -> bindings::HSteamPipe;
}

pub struct ClientManager {}

unsafe impl Manager for ClientManager {
    fn get_pipe() -> bindings::HSteamPipe {
        unsafe { bindings::SteamAPI_GetHSteamPipe() }
    }
}

impl Drop for ClientManager {
    fn drop(&mut self) {
        unsafe {
            bindings::SteamAPI_Shutdown();
        }
    }
}

struct SteamCtx<Manager> {
    _manager: Manager,
    callbacks: Mutex<Callbacks>,
}

struct Callbacks {
    callbacks: HashMap<i32, Box<dyn FnMut(*mut c_void) + Send + 'static>>,
    call_results:
        HashMap<bindings::SteamAPICall_t, Box<dyn FnOnce(*mut c_void, bool) + Send + 'static>>,
}

pub type SResult<T> = Result<T, SteamError>;
