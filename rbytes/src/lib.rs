use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Config {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config::from_args()
    }
}

fn count_bytes_from_buffer(buf: &mut impl Read, bytes_out: &mut u64) {
    for _ in buf.bytes() {
        *bytes_out += 1;
    }
}

fn count_bytes_from_file(file: &File, bytes_out: &mut u64) {
    *bytes_out += file.metadata().unwrap().len();
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.files.len() == 0 {
        let mut read_stream = BufReader::new(io::stdin());
        let mut count = 0;
        count_bytes_from_buffer(&mut read_stream, &mut count);
        println!("{}", count);
    } else {
        let mut total_count = 0;

        for path in &config.files {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("rbytes: {}: {}", path.to_str().unwrap(), e);
                    continue;
                }
            };

            let mut count = 0;
            count_bytes_from_file(&file, &mut count);
            println!("{} {}", count, path.to_str().unwrap());
            total_count += count;
        }

        if config.files.len() > 1 {
            println!("{} total", total_count);
        }
    }

    Ok(())
}
