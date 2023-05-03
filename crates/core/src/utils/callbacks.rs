use super::super::*;
use std::ffi::c_void;

pub unsafe trait Callback {
    const ID: i32;
    const SIZE: i32;
    unsafe fn from_raw(raw: *mut c_void) -> Self;
}

#[derive(Debug, Clone)]
pub struct EncryptedAppTicketResponse {
    pub result: bindings::EResult,
}
