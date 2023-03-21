use clap::{Parser,ValueEnum};
use std::{path::PathBuf, fs::File,collections::HashMap, io::{self, BufRead}};
use phf::phf_map;

static EscSeq: phf::Map<&'static str,Fromatting> = phf_map! {
    
};

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord,ValueEnum,Debug)]
enum OutputFormat {
    Text,
    Html,
}

#[derive(Parser,Debug)]
struct Cli {
    //This specifies the format that will be used to format the output.
    #[arg(long,short,value_enum,default_value_t=OutputFormat::Text)]
    format: OutputFormat,
    /// This specifes the output location of the programe if none is given then
    /// we will write to Standard Out.
    #[arg(long,short)]
    output: Option<String>,
    /// This specifes the files that we want to read in from to remove ANSI 
    /// formatting and replace it with something else at the end of files read
    /// here we will read from standard in.
    #[arg(value_name="FILE")]
    paths: Vec<PathBuf>,

}

struct TextPosistion {
    line: u64,
    char: u64,
}

impl TextPosistion {
    pub fn new() -> TextPosistion {
        TextPosistion {
            line: 0,
            char: 0,
        }
    }
}

enum Fromatting {
    Bold,
    Italic,
    Underline
}
/// This represent an area of text which as has some styling applied to it 
/// This this is read from an ANSI Control code which is formatted as such
/// 
/// \<ESC\> + "[" + <FMT_SEQ> + (";" + <FMT_SEQ>)
/// 
/// Where Esc is one of the following:
/// 
/// ^[
/// \033
/// \u001b
/// \x18
/// 
struct FormatBlock {
    start_posistion: TextPosistion,
    end_posistion: TextPosistion,
    style: Option<Fromatting>
}

impl FormatBlock {
    pub fn new() -> FormatBlock {
        FormatBlock { 
            start_posistion: TextPosistion::new(),
            end_posistion: TextPosistion::new(),
            style: None
        }
    }

    pub fn read_line() {

    }
}

struct FormatedTextFile {
    cursor: TextPosistion,
    text: Vec<String>,
    style_blocks: Vec<FormatBlock>
}

impl FormatedTextFile {
    pub fn new() -> FormatedTextFile {
        FormatedTextFile { 
            cursor: TextPosistion::new(),
            text: Vec::new(),
            style_blocks: Vec::new()
        }
    }

}

fn main() {
    let cli = Cli::parse();
    for path in cli.paths.iter() {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut fmt_txt = FormatedTextFile::new();
        for line in reader.lines() {
            
        }
    }
}
