use std::error::Error;

use tracing::info;
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::winnt::DLL_PROCESS_ATTACH,
};

// #[macro_use]
// extern crate paste;
#[macro_use]
extern crate detour3;
#[macro_use]
mod macros;

mod hooks;
mod logging;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: HINSTANCE, call_reason: DWORD, _: LPVOID) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        println!("Hello");
    }

    TRUE
}

pub fn initialize_monarch() -> Result<(), Box<dyn Error>> {
    info!("Start monarch init");
    // hooks::init_loading_hooks()?;
    // logging::init_logging_hooks()?;
    Ok(())
}
