use steamstacks_bindings as bindings;

pub mod callbacks;
pub mod result;

pub use callbacks::*;
pub use result::*;

#[derive(Clone)]
pub struct Utils {
    pub(crate) utils: *mut bindings::ISteamUtils,
}

unsafe impl Send for Utils {}
unsafe impl Sync for Utils {}

impl Utils {
    pub(crate) fn new() -> Self {
        Self {
            utils: unsafe { bindings::SteamAPI_SteamUtils_v010() },
        }
    }

    pub fn get_app_id(&self) -> u32 {
        unsafe { bindings::SteamAPI_ISteamUtils_GetAppID(self.utils) }
    }

    pub fn get_image_size(&self, handle: i32, width: &mut u32, height: &mut u32) -> bool {
        unsafe { bindings::SteamAPI_ISteamUtils_GetImageSize(self.utils, handle, width, height) }
    }

    pub fn get_image_rgba(&self, handle: i32, dest: &mut Vec<u8>, size: usize) -> bool {
        unsafe {
            bindings::SteamAPI_ISteamUtils_GetImageRGBA(
                self.utils,
                handle,
                dest.as_mut_ptr(),
                size as i32,
            )
        }
    }
}
