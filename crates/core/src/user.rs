use super::*;
use steamstacks_bindings as bindings;

pub struct User {
    pub(crate) user: *mut bindings::ISteamUser,
}

impl User {
    pub(crate) fn new() -> Self {
        Self {
            user: unsafe { bindings::SteamAPI_SteamUser_v022() },
        }
    }

    pub fn get_steam_id(&self) -> SteamId {
        SteamId(unsafe { bindings::SteamAPI_ISteamUser_GetSteamID(self.user) })
    }
}
