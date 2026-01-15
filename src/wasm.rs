use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_message(sender: &str, content: &str) -> String {
    let m = crate::create_message(sender, content);
    serde_json::to_string(&m).expect("serialize message")
}

#[wasm_bindgen]
pub fn send_message_json(json: &str) -> Result<(), JsValue> {
    crate::send_message_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
}