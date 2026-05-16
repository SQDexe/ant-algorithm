use {
    winapi::um::winnt::LANG_ENGLISH,
    winresource::WindowsResource,
    };

fn main() {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            .set_icon("./assets/icon.ico")
            .set_language(LANG_ENGLISH)
            .compile()
            .expect("Win settings build failed");
        }
    }