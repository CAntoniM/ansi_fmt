use clap::{Parser, ValueEnum};
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub mod ansi;
pub mod common;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Text,
    Html,
}

#[derive(Parser, Debug)]
struct Cli {
    //This specifies the format that will be used to format the output.
    #[arg(long,short,value_enum,default_value_t=OutputFormat::Text)]
    format: OutputFormat,
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

fn main() {
    let cli = Cli::parse();
    for path in cli.paths.iter() {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut ansi_text = ansi::Text::new();
        for line in reader.lines() {
            ansi_text.read(line.unwrap());
        }
    }
}
