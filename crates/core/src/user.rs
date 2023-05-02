use crate::steam_api::register_call_result;

use super::*;

#[derive(Clone)]
pub struct User {
    pub(crate) user: *mut bindings::ISteamUser,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}

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

    pub fn request_encrypted_app_ticket(&self) -> SResult<()> {
        unsafe {
            let api_call = bindings::SteamAPI_ISteamUser_RequestEncryptedAppTicket(
                self.user,
                std::ptr::null_mut(),
                0,
            );

            register_call_result::<bindings::EncryptedAppTicketResponse_t, _>(
                api_call,
                bindings::EncryptedAppTicketResponse_t_k_iCallback as i32,
                move |v, io_error| {
                    println!("Got call result: {:?} {:?}", v, io_error);
                },
            );
        }

        Ok(())
    }

    pub fn get_encrypted_app_ticket(&self) -> Result<(), &str> {
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

            println!("Ticket buffer: {:?}", ticket_buffer);
            Ok(())
        }
    }
}
