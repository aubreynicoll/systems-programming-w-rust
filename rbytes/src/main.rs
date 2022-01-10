use rbytes::Config;
use std::process;

fn main() {
    if let Err(e) = rbytes::run(&Config::new()) {
        eprintln!("rbytes: {}", e);
        process::exit(1);
    }
}
