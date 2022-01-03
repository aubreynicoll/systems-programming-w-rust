use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Cli {
    #[structopt(short = "n", help = "Remove trailing newline")]
    remove_newline: bool,
    string: Vec<String>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::from_args()
    }
}

pub fn run(args: Cli) {
    if args.remove_newline {
        print!("{}", args.string.join(" "));
    } else {
        println!("{}", args.string.join(" "));
    }
}
