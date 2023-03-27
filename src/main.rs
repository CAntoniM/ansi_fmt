use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub mod common;
pub mod input_fmt;
pub mod internal_format;
pub mod output_fmt;

#[derive(Parser, Debug)]
struct App {
    //This specifies the format that will be used to format the output.
    #[arg(long,short,value_enum,default_value_t=output_fmt::OutputFormat::Text)]
    format: output_fmt::OutputFormat,
    /// This specifes the output location of the programe if none is given then
    /// we will write to Standard Out.
    #[arg(long, short)]
    output: Option<String>,
    /// This specifes the files that we want to read in from to remove ANSI
    /// formatting and replace it with something else at the end of files read
    /// here we will read from standard in.
    #[arg(value_name = "FILE")]
    paths: Vec<PathBuf>,
}

impl App {
    pub fn run(&self) -> Result<(), &'static str> {
        for path in self.paths.iter() {
            let file = File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            let mut ansi_text = input_fmt::ansi::Text::new();
            for line in reader.lines() {
                ansi_text.read(line.unwrap());
            }
        }
        return Ok(());
    }
}

fn main() {
    let app = App::parse();
    let _ = app.run();
}
