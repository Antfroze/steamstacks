use super::*;
use core::ffi::CStr;
use steamstacks_bindings as bindings;

pub enum ImageSize {
    LARGE = 184,
    MEDIUM = 64,
    SMALL = 32,
}

pub struct Friends {
    pub(crate) friends: *mut bindings::ISteamFriends,
}

impl Friends {
    pub(crate) fn new() -> Self {
        Self {
            friends: unsafe { bindings::SteamAPI_SteamFriends_v017() },
        }
    }

    pub fn get_persona_name(&self) -> String {
        unsafe {
            let name = bindings::SteamAPI_ISteamFriends_GetPersonaName(self.friends);
            let name = CStr::from_ptr(name);
            name.to_string_lossy().into_owned()
        }
    }

    pub fn get_friends(&self, flags: FriendFlags) -> Vec<Friend> {
        let count = unsafe {
            bindings::SteamAPI_ISteamFriends_GetFriendCount(self.friends, flags.bits() as _)
        };

        if count <= 0 {
            return Vec::new();
        }

        let mut friends = Vec::with_capacity(count as usize);

        for idx in 0..count {
            let friend = SteamId(unsafe {
                bindings::SteamAPI_ISteamFriends_GetFriendByIndex(
                    self.friends,
                    idx,
                    flags.bits() as _,
                )
            });

            friends.push(Friend {
                id: friend,
                friends: self.friends,
            });
        }

        friends
    }

    pub fn get_avatar(&self, id: SteamId, size: ImageSize) -> Option<Vec<u8>> {
        let utils = steam_api::utils();

        let img = match size {
            ImageSize::LARGE => unsafe {
                bindings::SteamAPI_ISteamFriends_GetLargeFriendAvatar(self.friends, id.0)
            },
            ImageSize::MEDIUM => unsafe {
                bindings::SteamAPI_ISteamFriends_GetMediumFriendAvatar(self.friends, id.0)
            },
            ImageSize::SMALL => unsafe {
                bindings::SteamAPI_ISteamFriends_GetSmallFriendAvatar(self.friends, id.0)
            },
        };

        if img == 0 {
            return None;
        }

        let mut width = 0;
        let mut height = 0;

        if !utils.get_image_size(img, &mut width, &mut height) {
            return None;
        }

        let size_number = size as usize;
        assert_eq!(width, size_number as u32);
        assert_eq!(height, size_number as u32);
        let mut dest = vec![0; size_number * size_number * 4];

        if !utils.get_image_rgba(img, &mut dest, size_number * size_number * 4) {
            return None;
        }

        Some(dest)
    }
}

pub struct Friend {
    id: SteamId,
    friends: *mut bindings::ISteamFriends,
}

impl Friend {
    pub fn get_persona_name(&self) -> String {
        unsafe {
            let name =
                bindings::SteamAPI_ISteamFriends_GetFriendPersonaName(self.friends, self.id.0);
            let name = CStr::from_ptr(name);
            name.to_string_lossy().into_owned()
        }
    }

    pub fn get_steam_id(&self) -> SteamId {
        self.id
    }
}
