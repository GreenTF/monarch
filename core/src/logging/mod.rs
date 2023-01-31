
static_detour! {
    static: HelloHook: fn();
}

unsafe fn enable_hello() {
    // HelloHook;
}

fn init() {
    unsafe {enable_hello()};
}
