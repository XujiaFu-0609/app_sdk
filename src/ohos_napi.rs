#[cfg(feature = "ohos_napi")]
mod napi_mod {
    use crate::{create_message, send_message_json};
    use napi_derive_ohos::napi;
    use napi_ohos::bindgen_prelude::*;

    #[napi]
    pub fn createMessage(sender: String, content: String) -> String {
        let m = create_message(&sender, &content);
        serde_json::to_string(&m).unwrap_or_default()
    }

    #[napi]
    pub fn sendMessageJson(json: String) -> Result<()> {
        send_message_json(&json).map_err(|e| Error::from_reason(e.to_string()))
    }
}
