#[link(name = "monarch")]
extern "C" {
    fn ping() -> bool;
}

#[test]
fn call_ping_function() {
    unsafe { assert!(ping()) };
}
