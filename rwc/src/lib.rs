use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::ops;
use std::path::PathBuf;
use structopt::StructOpt;

type FnResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Config {
    #[structopt(short = "c", help = "Count bytes", overrides_with = "chars")]
    bytes: bool,
    #[structopt(short = "m", help = "Count UTF-8 chars", overrides_with = "bytes")]
    chars: bool,
    #[structopt(short = "w", help = "Count words")]
    words: bool,
    #[structopt(short = "l", help = "Count lines")]
    lines: bool,
    #[structopt(parse(from_os_str), name = "FILE", help = "Input file(s)")]
    files: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}

#[derive(Debug)]
// (lines, words, bytes/chars)
struct Count(u64, u64, u64);

impl Count {
    pub fn new() -> Self {
        Count(0, 0, 0)
    }

    pub fn count_from_bytes(input: &mut dyn BufRead) -> Self {
        let mut count = Self::new();
        let mut in_word = true;
        for byte in input.bytes() {
            let byte = byte.unwrap();
            if byte == '\n' as u8 {
                count.0 += 1;
            }
            if byte.is_ascii_whitespace() {
                if in_word {
                    count.1 += 1;
                    in_word = false;
                }
            } else if !in_word {
                in_word = true;
            }
            count.2 += 1;
        }
        count
    }

    pub fn count_from_chars(input: &mut dyn BufRead) -> Self {
        let mut count = Self::new();
        let mut buf = String::new();
        while input.read_line(&mut buf).unwrap() != 0 {
            for char in buf.chars() {
                if char == '\n' {
                    count.0 += 1;
                }
                if char.is_whitespace() {
                    count.1 += 1;
                }
                count.2 += 1;
            }
            buf.clear();
        }
        count
    }

    pub fn print(&self, config: &Config, label: Option<String>) {
        let no_args = !(config.bytes || config.chars || config.words || config.lines);

        if no_args || config.lines {
            print!("{:8}", self.0);
        }
        if no_args || config.words {
            print!("{:8}", self.1);
        }
        if no_args || config.chars || config.bytes {
            print!("{:8}", self.2);
        }
        if let Some(label) = label {
            println!(" {}", label);
        } else {
            println!();
        }
    }
}

impl ops::Add<Count> for Count {
    type Output = Self;

    fn add(self, other: Count) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign<Count> for Count {
    fn add_assign(&mut self, other: Count) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

pub fn run(config: &Config) -> FnResult<()> {
    let get_count: Box<dyn Fn(&mut dyn BufRead) -> Count> = match config.chars {
        true => Box::new(Count::count_from_chars),
        false => Box::new(Count::count_from_bytes),
    };

    if config.files.len() == 0 {
        let mut read_stream = BufReader::new(io::stdin());
        let count = get_count(&mut read_stream);
        count.print(config, None);
    } else {
        let mut total_count = Count::new();

        for path in &config.files {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("rwc: {}: {}", path.to_str().unwrap(), e);
                    continue;
                }
            };

            let mut read_stream = BufReader::new(file);
            let count = get_count(&mut read_stream);
            count.print(config, Some(String::from(path.to_str().unwrap())));
            total_count += count;
        }

        if config.files.len() > 1 {
            total_count.print(config, Some(String::from("total")));
        }
    }

    Ok(())
}
