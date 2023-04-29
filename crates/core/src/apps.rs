use super::*;
use crate::bindings;

pub struct Apps {
    pub(crate) apps: *mut bindings::ISteamApps,
}

impl Apps {
    pub(crate) fn new() -> Self {
        Self {
            apps: unsafe { bindings::SteamAPI_SteamApps_v008() },
        }
    }

    pub fn is_subscribed(&self) -> bool {
        {
            unsafe { bindings::SteamAPI_ISteamApps_BIsSubscribed(self.apps) }
        }
    }

    pub fn is_subscribed_app(&self, id: AppId) -> bool {
        {
            unsafe { bindings::SteamAPI_ISteamApps_BIsSubscribedApp(self.apps, id.0) }
        }
    }
}
