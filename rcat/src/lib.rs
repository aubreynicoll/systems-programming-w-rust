use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

const MAX_CONSECUTIVE_NEWLINES: u32 = 2;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Cli {
    #[structopt(short = "b", help = "Number non-blank lines. Overrides \"-n\"")]
    number_non_blank_lines: bool,
    #[structopt(short = "e", help = "Append dollar sign to each line")]
    append_dollar_sign: bool,
    #[structopt(short = "n", help = "Number lines. Incompatible with \"-b\"")]
    number_lines: bool,
    #[structopt(
        short = "s",
        help = "Reduce contiguous newline characters to maximum of 2"
    )]
    squeeze_empty_lines: bool,
    #[structopt(parse(from_os_str), name = "file")]
    files: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // get write stream
        let mut output = BufWriter::new(io::stdout());

        if self.files.len() == 0 {
            // read from stdin
            let mut input = BufReader::new(io::stdin());
            self.write_to_output(&mut input, &mut output)?;
        } else {
            // read from files/stdin
            for file in &self.files {
                let mut input: Box<dyn BufRead> = match file.to_str().unwrap() {
                    "-" => Box::new(BufReader::new(io::stdin())),
                    _ => Box::new(BufReader::new(File::open(file)?)),
                };
                self.write_to_output(&mut input, &mut output)?;
            }
        }

        // flush write stream before it is dropped
        output.flush()?;
        Ok(())
    }

    fn write_to_output(
        &self,
        input: &mut impl io::BufRead,
        output: &mut impl io::Write,
    ) -> Result<(), Box<dyn Error>> {
        let mut line_count: u32 = 0;
        let mut consecutive_newlines: u32 = 1; // treat new input as having 1 previous newline

        // read lines via String buffer
        let mut buf = String::new();
        while input.read_line(&mut buf)? != 0 {
            // handle line numbering
            if self.number_non_blank_lines {
                if buf.chars().next() != Some('\n') {
                    line_count += 1;
                    write!(output, "{:width$}\t", line_count, width = 6)?;
                }
            } else if self.number_lines {
                line_count += 1;
                write!(output, "{:width$}\t", line_count, width = 6)?;
            }

            // write to output
            for c in buf.chars() {
                // handle squeeze empty lines
                if self.squeeze_empty_lines {
                    if c == '\n' {
                        consecutive_newlines += 1;
                    } else {
                        consecutive_newlines = 0;
                    }

                    if consecutive_newlines > MAX_CONSECUTIVE_NEWLINES {
                        continue;
                    }
                }

                // handle append dollar sign
                if self.append_dollar_sign && c == '\n' {
                    write!(output, "$")?;
                }

                write!(output, "{}", c)?;
            }
            buf.clear();
        }

        Ok(())
    }
}
