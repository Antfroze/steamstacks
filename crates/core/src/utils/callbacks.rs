use super::super::*;
use std::{
    ffi::c_void,
    sync::{Arc, Weak},
};
use steamstacks_bindings as bindings;

pub unsafe trait Callback {
    const ID: i32;
    const SIZE: i32;
    unsafe fn from_raw(raw: *mut c_void) -> Self;
}

/// A handle that can be used to remove a callback
/// at a later point.
///
/// Removes the callback when dropped
pub struct CallbackHandle<Manager = ClientManager> {
    id: i32,
    ctx: Weak<SteamCtx<Manager>>,
}
unsafe impl<Manager> Send for CallbackHandle<Manager> {}

impl<Manager> Drop for CallbackHandle<Manager> {
    fn drop(&mut self) {
        if let Some(inner) = self.ctx.upgrade() {
            match inner.callbacks.lock() {
                Ok(mut cb) => {
                    cb.callbacks.remove(&self.id);
                }
                Err(err) => {
                    eprintln!("error while dropping callback: {:?}", err);
                }
            }
        }
    }
}

pub(crate) unsafe fn register_callback<C, F, Manager>(
    ctx: &Arc<SteamCtx<Manager>>,
    mut f: F,
) -> CallbackHandle<Manager>
where
    C: Callback,
    F: FnMut(C) + Send + 'static,
{
    {
        let mut callbacks = ctx.callbacks.lock().unwrap();
        callbacks.callbacks.insert(
            C::ID,
            Box::new(move |param| {
                let param = C::from_raw(param);
                f(param)
            }),
        );
    }
    CallbackHandle {
        id: C::ID,
        ctx: Arc::downgrade(&ctx),
    }
}

pub(crate) unsafe fn register_call_result<C, F, Manager>(
    ctx: &Arc<SteamCtx<Manager>>,
    api_call: bindings::SteamAPICall_t,
    _callback_id: i32,
    f: F,
) where
    F: for<'a> FnOnce(&'a C, bool) + 'static + Send,
{
    let mut callbacks = ctx.callbacks.lock().unwrap();
    callbacks.call_results.insert(
        api_call,
        Box::new(move |param, failed| f(&*(param as *const C), failed)),
    );
}
