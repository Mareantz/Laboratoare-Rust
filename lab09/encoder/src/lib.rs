use clap::Parser;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), about = "")]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "you 👀")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
//idk what to do in this crate
pub fn run() {
    println!(
        "encoder, version {}, built for {}",
        env!("CARGO_PKG_VERSION"),
        if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else if cfg!(target_os = "macos") {
            "Mac"
        } else {
            "an unknown OS"
        }
    );
}
