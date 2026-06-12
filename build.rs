use {
    winapi::um::winnt::LANG_ENGLISH,
    winresource::WindowsResource,
    std::env::var
    };



fn main() {
    if let Ok("windows") = var("CARGO_CFG_TARGET_OS").as_deref() {
        WindowsResource::new()
            .set_icon_with_id("./assets/icon.ico", "APP_ICON")
            .set_language(LANG_ENGLISH)
            .compile()
            .expect("Win settings build failed");
        }
    }