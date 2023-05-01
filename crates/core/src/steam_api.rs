use std::{collections::HashMap, ffi::c_void, sync::Mutex};

use crate::callbacks::Callback;

use super::*;

struct Callbacks {
    callbacks: HashMap<i32, Box<dyn FnMut(*mut c_void) + Send + 'static>>,
    call_results:
        HashMap<bindings::SteamAPICall_t, Box<dyn FnOnce(*mut c_void, bool) + Send + 'static>>,
}

lazy_static! {
    static ref USER: Mutex<Option<User>> = Mutex::new(None);
    static ref APPS: Mutex<Option<Apps>> = Mutex::new(None);
    static ref FRIENDS: Mutex<Option<Friends>> = Mutex::new(None);
    static ref UTILS: Mutex<Option<Utils>> = Mutex::new(None);
    static ref CLIENT_CALLBACKS: Mutex<Option<Callbacks>> = Mutex::new(None);
}

pub fn init() -> SResult<()> {
    unsafe {
        if !bindings::SteamAPI_Init() {
            return Err(SteamError::InitFailed);
        }
        bindings::SteamAPI_ManualDispatch_Init();

        let mut user_ref = USER.lock().unwrap();
        *user_ref = Some(User::new());

        let mut apps_ref = APPS.lock().unwrap();
        *apps_ref = Some(Apps::new());

        let mut friends_ref = FRIENDS.lock().unwrap();
        *friends_ref = Some(Friends::new());

        let mut utils_ref = UTILS.lock().unwrap();
        *utils_ref = Some(Utils::new());

        let mut callbacks_ref = CLIENT_CALLBACKS.lock().unwrap();
        *callbacks_ref = Some(Callbacks {
            callbacks: HashMap::new(),
            call_results: HashMap::new(),
        });

        Ok(())
    }
}

pub fn shutdown() {
    unsafe { bindings::SteamAPI_Shutdown() }
}

pub fn friends() -> Friends {
    let option = FRIENDS.lock().unwrap().to_owned();

    option.unwrap()
}

pub fn user() -> User {
    let option = USER.lock().unwrap().to_owned();

    option.unwrap()
}

pub fn apps() -> Apps {
    let option = APPS.lock().unwrap().to_owned();

    option.unwrap()
}

pub fn utils() -> Utils {
    let option = UTILS.lock().unwrap().to_owned();

    option.unwrap()
}

pub fn run_callbacks() {
    run_client_callbacks();
    // run_server_callbacks();
}

pub(crate) fn run_client_callbacks() {
    unsafe {
        let pipe = bindings::SteamAPI_GetHSteamPipe();
        bindings::SteamAPI_ManualDispatch_RunFrame(pipe);
        let mut callback = std::mem::zeroed();
        while bindings::SteamAPI_ManualDispatch_GetNextCallback(pipe, &mut callback) {
            let mut callbacks_ref = CLIENT_CALLBACKS.lock().unwrap();

            let callbacks = callbacks_ref.as_mut().unwrap();
            if callback.m_iCallback == bindings::SteamAPICallCompleted_t_k_iCallback as i32 {
                let apicall =
                    &mut *(callback.m_pubParam as *mut _ as *mut bindings::SteamAPICallCompleted_t);
                let mut apicall_result = vec![0; apicall.m_cubParam as usize];
                let mut failed = false;

                println!("{:?}", callback.m_iCallback as i32);

                if bindings::SteamAPI_ManualDispatch_GetAPICallResult(
                    pipe,
                    apicall.m_hAsyncCall,
                    apicall_result.as_mut_ptr() as *mut _,
                    apicall.m_cubParam as _,
                    apicall.m_iCallback,
                    &mut failed,
                ) {
                    // The &{val} pattern here is to avoid taking a reference to a packed field
                    // Since the value here is Copy, we can just copy it and borrow the copy
                    if let Some(cb) = callbacks.call_results.remove(&{ apicall.m_hAsyncCall }) {
                        cb(apicall_result.as_mut_ptr() as *mut _, failed);
                    }
                }
            } else {
                if let Some(cb) = callbacks.callbacks.get_mut(&callback.m_iCallback) {
                    cb(callback.m_pubParam as *mut _);
                }
            }
            bindings::SteamAPI_ManualDispatch_FreeLastCallback(pipe);
        }
    }
}

pub(crate) unsafe fn register_callback<C, F>(mut f: F)
where
    C: Callback,
    F: FnMut(C) + Send + 'static,
{
    let mut callbacks_ref = CLIENT_CALLBACKS.lock().unwrap();

    let callbacks = callbacks_ref.as_mut().unwrap();
    callbacks.callbacks.insert(
        C::ID,
        Box::new(move |param| {
            let param = C::from_raw(param);
            f(param)
        }),
    );
}

pub(crate) unsafe fn register_call_result<C, F>(
    api_call: bindings::SteamAPICall_t,
    _callback_id: i32,
    f: F,
) where
    F: for<'a> FnOnce(&'a C, bool) + 'static + Send,
{
    let mut callbacks_ref = CLIENT_CALLBACKS.lock().unwrap();

    let callbacks = callbacks_ref.as_mut().unwrap();
    callbacks.call_results.insert(
        api_call,
        Box::new(move |param, failed| f(&*(param as *const C), failed)),
    );
}
