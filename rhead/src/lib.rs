use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

type FnResult = Result<(), Box<dyn Error>>;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Config {
    #[structopt(
        short = "n",
        long = "lines",
        help = "Number of lines to read",
        name = "LINES",
        conflicts_with = "BYTES"
    )]
    lines: Option<usize>,
    #[structopt(
        short = "c",
        long = "bytes",
        help = "Number of bytes to read",
        name = "BYTES",
        conflicts_with = "LINES"
    )]
    bytes: Option<usize>,
    #[structopt(parse(from_os_str), name = "FILE", help = "Input file(s)")]
    files: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}

pub fn run(config: &Config) -> FnResult {
    let (print_to_stream, limit): (
        Box<dyn Fn(&mut dyn BufRead, &mut dyn Write, usize) -> FnResult>,
        usize,
    ) = match config.bytes {
        Some(bytes) => (Box::new(print_bytes), bytes),
        _ => match config.lines {
            Some(lines) => (Box::new(print_lines), lines),
            _ => (Box::new(print_lines), 10),
        },
    };

    let mut write_stream = BufWriter::new(io::stdout());

    if config.files.len() == 0 {
        // read stdin
        let mut read_stream = BufReader::new(io::stdin());
        print_to_stream(&mut read_stream, &mut write_stream, limit)?;
    } else {
        // read files
        for (index, filepath) in config.files.iter().enumerate() {
            let file = match File::open(filepath) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("rhead: {}: {}", filepath.to_str().unwrap(), e);
                    continue;
                }
            };

            let mut read_stream = BufReader::new(file);

            if config.files.len() > 1 {
                // write header
                if index > 0 {
                    writeln!(write_stream)?;
                }
                writeln!(write_stream, "==> {} <==", filepath.to_str().unwrap())?;
            }

            print_to_stream(&mut read_stream, &mut write_stream, limit)?;
        }
    }

    write_stream.flush()?;
    Ok(())
}

fn print_bytes(input: &mut dyn BufRead, output: &mut dyn Write, num_bytes: usize) -> FnResult {
    for byte in input.bytes().take(num_bytes) {
        let byte = byte?;
        write!(output, "{}", byte as char)?;
    }
    Ok(())
}

fn print_lines(input: &mut dyn BufRead, output: &mut dyn Write, num_lines: usize) -> FnResult {
    for line in input.lines().take(num_lines) {
        let line = line?;
        writeln!(output, "{}", line)?;
    }
    Ok(())
}
