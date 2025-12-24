use {
    std::env::var,
    winapi::um::winnt::LANG_ENGLISH,
    winresource::WindowsResource,
    };

fn main() {
    if Ok("windows") == var("CARGO_CFG_TARGET_OS").as_deref() {
        WindowsResource::new()
            .set_icon("./assets/icon.ico")
            .set_language(LANG_ENGLISH)
            .compile()
            .expect("Win settings build failed");
        }
    }