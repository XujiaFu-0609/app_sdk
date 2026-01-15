fn main() {
    #[cfg(target_env = "ohos")]
    {
        ohos_ndk_env::setup_ohos_ndk_env();
    }
}
