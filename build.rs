use {
    std::{
        env::var_os,
        io::Result as IO_Result
        },
    winresource::WindowsResource,
    };

fn main() -> IO_Result<()> {
    if var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
        }
    Ok(())
    }