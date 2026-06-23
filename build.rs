use {
    winapi::um::winnt::LANG_ENGLISH,
    winresource::WindowsResource,
    std::env::var
    };



fn main() {
    if var("CARGO_CFG_TARGET_OS").is_ok_and(|e| e == "windows") {
        WindowsResource::new()
            .set_icon("./assets/icon.ico")
            .set_language(LANG_ENGLISH)
            .compile()
            .expect("Win settings build failed");
        }
    }