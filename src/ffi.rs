use safer_ffi::prelude::*;

use crate::{create_message, send_message, send_message_json};

#[derive_ReprC]
#[repr(C)]
pub struct MessageParams<'a> {
    pub sender: char_p::Ref<'a>,
    pub content: char_p::Ref<'a>,
}

#[ffi_export]
fn app_create_message<'a>(params: &'a MessageParams<'a>) -> char_p::Box {
    let sender = params.sender.to_str();
    let content = params.content.to_str();
    let m = create_message(sender, content);
    let json = serde_json::to_string(&m).unwrap_or_else(|_| "{}".to_string());
    char_p::new(json)
}

#[ffi_export]
fn app_send_message_json(json: char_p::Ref<'_>) -> i32 {
    let json = json.to_str();
    match send_message_json(json) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

#[ffi_export]
fn app_send_message<'a>(params: &'a MessageParams<'a>) -> i32 {
    let sender = params.sender.to_str();
    let content = params.content.to_str();
    let m = create_message(sender, content);
    match send_message(&m) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}

#[ffi_export]
fn app_string_free(s: char_p::Box) {
    drop(s)
}

#[cfg(feature = "headers")]
#[test]
fn generate_headers() -> std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("target/app_sdk.h")?
        .generate()
}