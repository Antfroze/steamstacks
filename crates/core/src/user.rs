use super::*;

#[derive(Clone)]
pub struct User {
    pub(crate) user: *mut bindings::ISteamUser,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}

impl User {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                user: bindings::SteamAPI_SteamUser_v022(),
            }
        }
    }

    /// Get the steam id of the current logged in user
    pub fn get_steam_id(&self) -> SteamId {
        unsafe { SteamId(bindings::SteamAPI_ISteamUser_GetSteamID(self.user)) }
    }

    pub fn request_encrypted_app_ticket(&self) -> u64 {
        unsafe {
            let api_call = bindings::SteamAPI_ISteamUser_RequestEncryptedAppTicket(
                self.user,
                std::ptr::null_mut(),
                0,
            );

            api_call
        }
    }

    /// Retrieve a encrypted app ticket
    /// If called without calling `request_encryped_app_ticket()`, will likely result in
    /// stale ticket.
    pub fn get_encrypted_app_ticket(&self) -> Result<Vec<i32>, &str> {
        unsafe {
            let mut ticket_buffer = vec![0; 1024];
            let mut ticket_len = 0;

            if !bindings::SteamAPI_ISteamUser_GetEncryptedAppTicket(
                self.user,
                ticket_buffer.as_mut_ptr() as *mut _,
                1024,
                &mut ticket_len,
            ) {
                return Err("Failed to get encrypted app ticket");
            };

            ticket_buffer.truncate(ticket_len as usize);

            Ok(ticket_buffer)
        }
    }
}
