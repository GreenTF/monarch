macro_rules! load_addr {
    ($mod_name:literal + $offset:expr) => {
        unsafe {
            use libloading::os::windows::Library;
            // use libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH;
            use tracing::debug;

            debug!("Loading module {}", $mod_name);
            match Library::open_already_loaded($mod_name) {
                Ok(module) => {
                    debug!("loaded module {}", $mod_name);
                    let handle = module.into_raw();
                    Ok(handle.offset($offset))
                }
                Err(e) => Err(e),
            }
        }
    };

    ($mod_name:literal, $proc_name:literal, $ty:ty) => {
        unsafe {
            use libloading::os::windows::Library;
            // use libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH;
            use tracing::debug;

            debug!("Loading module {}", $mod_name);
            match Library::open_already_loaded($mod_name) {
                Ok(module) => {
                    debug!("loaded module {}", $mod_name);
                    module.get::<$ty>($proc_name)
                }
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! dll_callback {
    ($mod_name:literal, $func:expr) => {{
        let mut list = $crate::hooks::DLL_CALLBACKS
            .lock()
            .expect("Unable to lock callback list");

        if let Some(cbs) = list.get_mut($mod_name) {
            cbs.push($func);
        } else {
            list.insert($mod_name.into(), vec![$func]);
        }
    }};
}
