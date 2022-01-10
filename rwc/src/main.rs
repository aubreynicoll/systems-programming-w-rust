use rwc::Config;
use std::process;

fn main() {
    if let Err(e) = rwc::run(&Config::new()) {
        eprintln!("rwc: {}", e);
        process::exit(1);
    }
}
