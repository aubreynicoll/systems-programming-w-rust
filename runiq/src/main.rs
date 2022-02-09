use runiq::Cli;

fn main() {
    if let Err(e) = runiq::run(&Cli::new()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
