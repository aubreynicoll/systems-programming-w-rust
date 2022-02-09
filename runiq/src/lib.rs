use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Cli {
    #[structopt(
        name = "PREPEND_COUNT",
        short = "c",
        long = "count",
        help = "Show the number of occurrences of each line"
    )]
    prepend_count: bool,

    #[structopt(parse(from_os_str), name = "INPUT", help = "Input file path")]
    input_filepath: Option<PathBuf>,

    #[structopt(parse(from_os_str), name = "OUTPUT", help = "Output file path")]
    output_filepath: Option<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}

pub fn run(args: &Cli) -> Result<(), Box<dyn Error>> {
    // set up io
    let read_buf: BufReader<Box<dyn Read>> = match &args.input_filepath {
        Some(filepath) if filepath.to_str().unwrap() != "-" => {
            BufReader::new(Box::new(File::open(filepath)?))
        }
        _ => BufReader::new(Box::new(io::stdin())),
    };

    let mut write_buf: BufWriter<Box<dyn Write>> = match &args.output_filepath {
        Some(filepath) => BufWriter::new(Box::new(File::create(filepath)?)),
        _ => BufWriter::new(Box::new(io::stdout())),
    };

    // count & print duplicate lines
    let mut count: u32 = 0;
    let mut prev_line: Option<String> = None;

    for line in read_buf.lines() {
        let curr_line = line?;

        if let Some(prev_line) = prev_line {
            if curr_line != prev_line {
                if args.prepend_count {
                    let formatted = format!("{:4} ", count);
                    write!(write_buf, "{}", formatted)?;
                }
                writeln!(write_buf, "{}", prev_line)?;
                count = 0;
            }
        }

        count += 1;
        prev_line = Some(curr_line);
    }

    // handle last line
    if let Some(prev_line) = prev_line {
        if args.prepend_count {
            let formatted = format!("{:4} ", count);
            write!(write_buf, "{}", formatted)?;
        }
        writeln!(write_buf, "{}", prev_line)?;
    }

    write_buf.flush()?;
    Ok(())
}
