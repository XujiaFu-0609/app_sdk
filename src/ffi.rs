#[allow(unused_imports)]
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use crate::{create_message, send_message, send_message_json};

#[no_mangle]
pub unsafe extern "C" fn app_create_message(sender: *const c_char, content: *const c_char) -> *mut c_char {
    let sender = CStr::from_ptr(sender).to_string_lossy().into_owned();
    let content = CStr::from_ptr(content).to_string_lossy().into_owned();
    let m = create_message(&sender, &content);
    let json = serde_json::to_string(&m).unwrap_or_else(|_| "{}".to_string());
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn app_send_message_json(json: *const c_char) -> c_int {
    let json = CStr::from_ptr(json).to_string_lossy().into_owned();
    match send_message_json(&json) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn app_send_message(sender: *const c_char, content: *const c_char) -> c_int {
    let sender = CStr::from_ptr(sender).to_string_lossy().into_owned();
    let content = CStr::from_ptr(content).to_string_lossy().into_owned();
    let m = create_message(&sender, &content);
    match send_message(&m) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn app_string_free(s: *mut c_char) {
    if s.is_null() { return; }
    let _ = CString::from_raw(s);
}
