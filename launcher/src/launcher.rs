use anyhow::Result;
use lazy_static::lazy_static;
use libloading::os::windows::{Library, LOAD_WITH_ALTERED_SEARCH_PATH};
use std::{ffi::c_void, path::PathBuf, ptr::null};
use tracing::{debug, error};

lazy_static! {
    static ref EXE_DIR: PathBuf = std::env::current_exe().expect("Unable to get the path to the executable").parent().expect("Executable path has no parent dir").to_path_buf();
    // needs to be static so that it doesn't get dropped until the end of the program
    static ref TIER0: Result<Library, libloading::Error> = unsafe {
        let path = EXE_DIR.join("bin").join("x64_retail").join("tier0.dll");
        debug!("loading tier0 from path {}", path.display());
        #[cfg(windows)]
        Library::load_with_flags(path, LOAD_WITH_ALTERED_SEARCH_PATH)
    };
    static ref NSDLL: Result<Library, libloading::Error> = unsafe{
        let path = EXE_DIR.join("Northstar.dll");
        debug!("loading Northstar.dll from path {}", path.display());
        #[cfg(windows)]
        Library::load_with_flags(path, LOAD_WITH_ALTERED_SEARCH_PATH)
    };
    static ref LAUNCHER: Result<Library, libloading::Error> =  unsafe {
        let path = EXE_DIR.join("bin").join("x64_retail").join("launcher.dll");
        debug!("loading launcher from path {}", path.display());
        #[cfg(windows)]
        Library::load_with_flags(path, LOAD_WITH_ALTERED_SEARCH_PATH)
    };
}

pub fn init() -> Result<()> {
    // set the current directory
    if let Err(e) = std::env::set_current_dir(EXE_DIR.as_path()){
        error!("Unable to set working directory to executable's current dir");
        error!("{e}");
    }
    load_tier0()?;
    load_northstar()?;
    monarch::initialize_monarch().expect("Monarch init failed");
    start_launcher()?;

    Ok(())
}

fn start_launcher() -> Result<()> {
    match &*LAUNCHER {
        Ok(lib) => {
            let main = unsafe {
                lib.get::<extern "C" fn(*const c_void, *const c_void, *const c_void, i32) -> ()>(
                    b"LauncherMain",
                )?
            };

            main(null(), null(), null(), 0);

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

fn load_northstar() -> Result<()> {
    match &*NSDLL {
        Ok(lib) => {
            let main = unsafe { lib.get::<extern "C" fn() -> bool>(b"InitialiseNorthstar")? };
            debug!("Calling InitialiseNorthstar");
            let res = main();
            debug!("Returned {res}");

            let plugins = unsafe { lib.get::<extern "C" fn() -> bool>(b"LoadPlugins")? };

            plugins();
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

fn load_tier0() -> Result<()> {
    if let Err(e) = &*TIER0 {
        Err(e.into())
    } else {
        Ok(())
    }
}
