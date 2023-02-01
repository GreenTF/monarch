use detour3::GenericDetour;
use tracing::debug;
use winapi::shared::minwindef::{HMODULE};

type TextMsgFn = extern "C" fn(*const u8);

// static_detour! {
//     static TextMsgHook: unsafe extern "C" fn(*const u8);
// }

extern "C" fn msg_handler(msg: *const u8) {
    {
        println!("-----");
        println!("{msg:?}");
        println!("-----");
    }
}

pub(super) fn enable_text_msg_hook(module: HMODULE) {
    debug!("enabling text msg hook");
    let addr = unsafe { module.offset(0x198710) };
    unsafe {
        let fun: TextMsgFn = std::mem::transmute(addr);
        GenericDetour::<TextMsgFn>::new(fun, msg_handler)
            .expect("Unable to initialize client logging hook")
            .enable()
            .expect("Unable to enable client logging hook");
    }
}
