use super::*;
use crate::bindings;

pub struct User {
    pub(crate) user: *mut bindings::ISteamUser,
}

impl User {
    pub(crate) fn new() -> Self {
        Self {
            user: unsafe { bindings::SteamAPI_SteamUser_v022() },
        }
    }

    /// Get the steam id of the current logged in user
    pub fn get_steam_id(&self) -> SteamId {
        SteamId(unsafe { bindings::SteamAPI_ISteamUser_GetSteamID(self.user) })
    }

    pub fn request_encrypted_app_ticket(&self) {
        unsafe { bindings::SteamAPI_ManualDispatch_GetAPICallResult() }
    }
}
