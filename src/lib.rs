use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
#[cfg(feature = "uniffi")]
pub struct UniFfiTag;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Message {
    pub id: u64,
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum AppError {
    InvalidInput(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidInput(s) => write!(f, "InvalidInput: {}", s),
            AppError::Internal(s) => write!(f, "Internal: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

pub fn create_message(sender: &str, content: &str) -> Message {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    Message {
        id,
        sender: sender.to_string(),
        content: content.to_string(),
        timestamp: ts,
    }
}

pub fn send_message(msg: &Message) -> Result<(), AppError> {
    // Demo: 在实际场景中这里会调用网络/队列/系统通道。
    // 这里仅打印模拟发送。
    println!(
        "Send message: id={} sender='{}' content='{}' ts={}",
        msg.id, msg.sender, msg.content, msg.timestamp
    );
    Ok(())
}

pub fn send_message_json(json: &str) -> Result<(), AppError> {
    let m: Message = serde_json::from_str(json)
        .map_err(|e| AppError::InvalidInput(format!("invalid json: {}", e)))?;
    send_message(&m)
}

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "ohos")]
pub mod ffi;

#[cfg(feature = "uniffi")]
mod uniffi_api {
    use super::*;


    #[uniffi::export]
    pub fn create_message(sender: String, content: String) -> Message {
        super::create_message(&sender, &content)
    }

    #[uniffi::export]
    pub fn send_message(msg: Message) {
        let _ = super::send_message(&msg);
    }
}