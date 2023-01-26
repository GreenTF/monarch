// this is here because I don't want it in the dynamic lib and there isn't currently
// support for coniditional compilation on the crate-type
mod launcher;

#[cfg(debug_assertions)]
fn main() {
    let res = launcher::init();
    println!("{:?}", res);
}

#[cfg(not(debug_assertions))]
fn main() {
    todo!();
}
