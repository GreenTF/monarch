use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::winnt::DLL_PROCESS_ATTACH,
};

#[macro_use]
extern crate paste;
#[macro_use]
extern crate detour3;
#[macro_use]
mod macros;

mod logging;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: HINSTANCE, call_reason: DWORD, _: LPVOID) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        println!("Hello");
    }

    TRUE
}
