use clap::Parser;
use std::{
    fs::File,
    io::{self, Write},
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
    pub fn write(&self, string: &String) -> Result<(), &'static str> {
        match self.output.clone() {
            Some(path) => {
                match std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path)
                {
                    Ok(mut file) => {
                        if let Err(_e) = file.write(string.as_bytes()) {
                            return Err("Failed to write to file");
                        }
                        return Ok(());
                    }
                    Err(_e) => {
                        return Err("failed to open file");
                    }
                }
            }
            None => {
                if let Err(_e) = std::io::stdout().write(string.as_bytes()) {
                    return Err("failed to write to standard out");
                }
                return Ok(());
            }
        }
    }

    pub fn parse_text(&self, string: String) -> Result<String, &'static str> {
        let mut ansi_text = input_fmt::ansi::Text::new();
        ansi_text.read(string);
        match output_fmt::from(self.format, internal_format::Text::from_ansi(ansi_text)) {
            Some(formater) => {
                return Ok(formater.to_string());
            }
            None => return Err("Failed to find a writer for the given output format."),
        }
    }

    pub fn run(&self) -> Result<(), &'static str> {
        for path in self.paths.iter() {
            let file = File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            if let Err(e) = self.parse_text(std::io::read_to_string(reader).unwrap()) {
                return Err(e);
            }
        }
        match self.parse_text(std::io::read_to_string(std::io::stdin()).unwrap()) {
            Ok(output_text) => {
                if let Err(e) = self.write(&output_text) {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}

fn main() -> Result<(), &'static str> {
    let app = App::parse();
    return app.run();
}

#[cfg(test)]
mod test {
    use crate::{output_fmt, App};

    #[test]
    pub fn app_parse_text() {
        let test_cases = [
            (
                (
                    "\x1b[93m\x1b[1mTest\x1b[0m",
                    App {
                        format: output_fmt::OutputFormat::Text,
                        output: Some("test.txt".to_string()),
                        paths: vec![],
                    },
                ),
                "Test",
            ),
            (
                (
                    "Test",
                    App {
                        format: output_fmt::OutputFormat::Text,
                        output: Some("test.txt".to_string()),
                        paths: vec![],
                    },
                ),
                "Test",
            ),
        ];
        for test_case in test_cases {
            let ((text, app), expected_result) = test_case;
            assert_eq!(
                app.parse_text(text.to_string()).unwrap(),
                expected_result.to_string()
            )
        }
    }
}
