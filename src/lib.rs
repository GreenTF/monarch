#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn ping() -> bool {
    true
}
