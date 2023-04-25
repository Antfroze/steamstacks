use steamstacks_bindings as bindings;

use crate::{apps::Apps, friends::Friends, user::User, utils::Utils};

pub fn init() -> Result<(), &'static str> {
    if !unsafe { bindings::SteamAPI_Init() } {
        return Err("SteamAPI Init failed.");
    }

    Ok(())
}

pub fn shutdown() {
    unsafe {
        bindings::SteamAPI_Shutdown();
    }
}

pub fn friends() -> Friends {
    Friends::new()
}

pub fn user() -> User {
    User::new()
}

pub fn apps() -> Apps {
    Apps::new()
}

pub fn utils() -> Utils {
    Utils::new()
}
