// this is here because I don't want it in the dynamic lib and there isn't currently
// support for coniditional compilation on the crate-type
mod launcher;

fn main() {
    use tracing::metadata::LevelFilter;

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(LevelFilter::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Unable to initialize tracing");
    let res = launcher::init();
    println!("{:?}", res);
}
