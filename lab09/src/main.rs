use base64::encode;
use encoder::run;
fn main() {
    let name = "Many hands make light work.".to_string();
    run();
    println!(
        "\"{}\" in base64 is {}",
        name,
        encode(&name.clone().into_bytes())
    )
}
