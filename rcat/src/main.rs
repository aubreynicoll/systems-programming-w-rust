use rcat::Cli;
use std::process;

fn main() {
    if let Err(e) = Cli::new().run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
