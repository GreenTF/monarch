use anyhow::Result;
use lazy_static::lazy_static;
use libloading::os::windows::{Library, LOAD_WITH_ALTERED_SEARCH_PATH};
use std::path::PathBuf;
use tracing::debug;

lazy_static! {
    static ref EXE_DIR: PathBuf = std::env::current_exe().expect("Unable to get the path to the executable").parent().expect("Executable path has no parent dir").to_path_buf();
    // needs to be static so that it doesn't get dropped until the end of the program
    static ref TIER0: Result<Library, libloading::Error> = unsafe {
        let path = EXE_DIR.join("bin").join("x64_retail").join("tier0.dll");
        debug!("loading tier0 from path {}", path.display());
        #[cfg(windows)]
        Library::load_with_flags(path, LOAD_WITH_ALTERED_SEARCH_PATH)
    };
}

pub fn init() -> Result<()> {
    load_tier0()?;

    Ok(())
}

fn load_tier0() -> Result<()> {
    if let Err(e) = &*TIER0 {
        Err(e.into())
    } else {
        Ok(())
    }
}
