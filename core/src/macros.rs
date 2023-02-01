macro_rules! load_addr {
    ($mod_name:literal + $offset:expr) => {
        unsafe {
            use libloading::os::windows::Library;
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

    ($mod_name:path, $proc_name:literal, $ty:ty) => {
        unsafe {
            $mod_name.get::<$ty>($proc_name)
        }
    };

    ($mod_name:literal, $proc_name:literal, $ty:ty) => {
        unsafe {
            use libloading::os::windows::Library;
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
    ($mod_name:literal, $tag:literal, $func:expr) => {{
        use std::collections::BTreeSet;
        let mut list = $crate::hooks::DLL_CALLBACKS
            .lock()
            .expect("Unable to lock callback list");

        let cb = $crate::hooks::Callback::new($tag, BTreeSet::new(), $func);

        if let Some(cbs) = list.get_mut($mod_name) {
            cbs.push(cb);
        } else {
            list.insert($mod_name.into(), vec![cb]);
        }
    }};
}

