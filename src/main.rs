use clap::{builder::Str, Parser};
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::PathBuf,
    sync::mpsc::channel, process::Output,
};
use threadpool::ThreadPool;

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
    ///This specifies the amount of threads that are meant launched process the data
    ///Just a note this will only provide more proformance if you have a large
    ///amount of threads.
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

    pub fn parse_text(format: &output_fmt::OutputFormat, string: String) -> Result<String, String> {
        let mut ansi_text = input_fmt::ansi::Text::new();
        ansi_text.read(string);
        match output_fmt::from(format.clone(), internal_format::Text::from_ansi(ansi_text)) {
            Some(formater) => {
                return Ok(formater.to_string());
            }
            None => return Err("Failed to find a writer for the given output format.".to_string()),
        }
    }

    pub fn run_async(paths: Vec<PathBuf>,threads: usize, format: output_fmt::OutputFormat) -> Result<Vec<String>, String> {
        let pool = ThreadPool::new(threads);
        let mut results: Vec<String> = Vec::new();
        let tp_fmt = format.clone();
        let (tx, rx) = channel();
        for path in paths.iter() {
            let tp_path = path.clone();
            let tp_tx = tx.clone();
            pool.execute(move || {
                let file = File::open(tp_path).unwrap();
                let reader = io::BufReader::new(file);
                match parse_text(&tp_fmt, std::io::read_to_string(reader).unwrap()) {
                    Ok(output_text) => {
                        tp_tx.send(Ok(output_text)).unwrap();
                    }
                    Err(e) => {
                        tp_tx.send(Err(e)).unwrap();
                    }
                }
            });
        }

        for result in rx.iter().take(paths.len()) {
            match result {
                Ok(output_txt) => {
                    results.push(output_txt);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(results);
    }

    pub fn run_stream(output: Option<String>, format: output_fmt::OutputFormat ) -> Result<(), String> {
        let mut out_writer = match &output {
            Some(x) => Box::new(File::create(x).unwrap()) as Box<dyn Write>,
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(txt) => match parse_text(&format, txt +"\n") {
                    Ok(output_text) => {
                        out_writer.write(output_text.as_bytes()).unwrap();
                    }
                    Err(e) => return Err(e),
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        return Ok(());
    }

fn main() -> Result<(), String> {
    let app = App::parse();
    let mut out_writer = match &app.output {
        Some(x) => Box::new(File::create(x).unwrap()) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    if app.paths.len() > 0  {
        match run_async(app.paths,app.threads,app.format) {
            Ok(output_text) => {
                for text  in output_text.iter() {
                    if let Err(e) = out_writer.write(text.as_bytes()) {
                        return Err(e.to_string());
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        match run_stream(app.output, app.format) {
            Ok(_) => {}
            Err(e) => {return Err(e)}
        }
    }

    return Ok(());
}

#[cfg(test)]
mod test {
    use crate::{output_fmt, App, parse_text};

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
                        threads: 1,
                    },
                ),
                "Test",
            ),
            (
                (
                    "\x1b[01;32mTest\x1b[0m",
                    App {
                        format: output_fmt::OutputFormat::Html,
                        output: Some("test.html".to_string()),
                        paths: vec![],
                        threads: 1,
                    },
                ),
                "<b><span style=\"color=#0800\">Test</b></span>",
            ),
            (
                (
                    "Test",
                    App {
                        format: output_fmt::OutputFormat::Text,
                        output: Some("test.txt".to_string()),
                        paths: vec![],
                        threads: 1,
                    },
                ),
                "Test",
            ),
        ];
        for test_case in test_cases {
            let ((text, app), expected_result) = test_case;
            let res = parse_text(&app.format, text.to_string());
            match res {
                Ok(r) => {
                    assert_eq!(r, expected_result.to_string())
                }
                Err(e) => {
                    println!("{}", e);
                    panic!("{}", e);
                }
            }
        }
    }
}
