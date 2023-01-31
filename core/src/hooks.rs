use std::{collections::BTreeMap, error::Error, os::windows::raw::HANDLE, sync::Mutex};

pub static DLL_CALLBACKS: Mutex<BTreeMap<String, Vec<fn(HMODULE)>>> = Mutex::new(BTreeMap::new());

use libloading::os::windows::Library;
use tracing::{debug, instrument};
use winapi::shared::minwindef::{DWORD, HMODULE};
// use winapi::um::winnt::LPCSTR;
use windows::core::PCSTR;

type ExAHookType = unsafe extern "system" fn(PCSTR, HANDLE, DWORD) -> HMODULE;
type AHookType = unsafe extern "system" fn(PCSTR) -> HMODULE;

// can't use the type alias here for reasons
static_detour! {
    static LoadLibExAHook: unsafe extern "system" fn(PCSTR, HANDLE, DWORD) -> HMODULE;
    static LoadLibAHook: unsafe extern "system" fn(PCSTR) -> HMODULE;
}

#[instrument]
fn load_lib_a(file_name: PCSTR) -> HMODULE {
    debug!("hooking LodLibraryA");

    unsafe {
        let module = LoadLibAHook.call(file_name);
        debug!(
            "dispatching callbacks for {}",
            file_name.to_string().unwrap_or_else(|_| "Unknown".into())
        );
        // if let Some(cbs) = DLL_CALLBACKS
        //     .lock()
        //     .expect("unable to lock callbacks list")
        //     .get(
        //         &file_name
        //             .to_string()
        //             .expect("Unable to transoform lib name into string"),
        //     )
        // {
        //     // for f in cbs {
        //     //     f(module);
        //     // }
        // }

        module
    }
}

#[instrument]
fn load_lib_ex_a(file_name: PCSTR, file: HANDLE, flags: DWORD) -> HMODULE {
    debug!("hooking LoadLibraryExA");

    unsafe {
        let module = LoadLibExAHook.call(file_name, file, flags);
        debug!(
            "dispatching callbacks for {}",
            file_name.to_string().unwrap_or_else(|_| "Unknown".into())
        );
        // if let Some(cbs) = DLL_CALLBACKS
        //     .lock()
        //     .expect("unable to lock callbacks list")
        //     .get(
        //         file_name
        //             .to_string()
        //             .expect("Unable to transoform lib name into string")
        //             .split('\\')
        //             .last()
        //             .unwrap(),
        //     )
        // {
        //     // for f in cbs {
        //     //     f(module);
        //     // }
        // }

        module
    }
}

pub fn init_loading_hooks() -> Result<(), Box<dyn Error>> {
    let kernel = unsafe { Library::new("kernel32.dll")? };
    let target = unsafe { kernel.get::<ExAHookType>(b"LoadLibraryExA")? };
    unsafe {
        LoadLibExAHook
            .initialize(*target, load_lib_ex_a)?
            .enable()?;
    }
    let target = unsafe { kernel.get::<AHookType>(b"LoadLibraryA")? };

    unsafe {
        LoadLibAHook.initialize(*target, load_lib_a)?.enable()?;
    }
    Ok(())
}
