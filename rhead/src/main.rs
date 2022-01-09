use rhead::Config;
use std::process;

fn main() {
    if let Err(e) = rhead::run(&Config::new()) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
