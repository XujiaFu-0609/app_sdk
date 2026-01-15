fn main() {
    #[cfg(feature = "ohos_napi")]
    {
        napi_build_ohos::setup();
    }
}
