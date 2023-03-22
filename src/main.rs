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

#[derive(Debug)]
struct TextPosistion {
    line: usize,
    char: usize,
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
}


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

}

struct FormatedTextFile {
    text: Vec<String>,
    style_blocks: Vec<FormatBlock>
}

impl FormatedTextFile {
    pub fn new() -> FormatedTextFile {
        FormatedTextFile { 
            text: Vec::new(),
            style_blocks: Vec::new()
        }
    }

    pub fn get_all_escape_sequances(line: &String,line_no:usize) -> Vec<TextPosistion> {
        let escape_sequances = [r"\e",r"\033",r"\u001b",r"\x1B",r"^[","\u{001b}"];        
        let matcher= |line : &String, token: &str| -> Vec<usize> {
            line.match_indices(token).map(|(i,_)|i).collect()
        };

        let mut escape_sequnce_posistion: Vec<TextPosistion> = Vec::new();
        let mut indexs: Vec<usize> = Vec::new();
        
        for escape_sequance in escape_sequances {
            let mut temp = matcher(line,escape_sequance);
            indexs.append(&mut temp);
        }

        indexs.sort();
        
        for index in indexs {
            escape_sequnce_posistion.push(TextPosistion { line: line_no, char: index});
        }
        
        escape_sequnce_posistion
    }

    pub fn read_line(&mut self, line: String) {
        let escape_posistions = FormatedTextFile::get_all_escape_sequances(&line, self.text.len());
        
        for escape_posistion in escape_posistions {
            println!("{:#?}",escape_posistion);
        }
        self.text.push(line);
    }

}

fn main() {
    let cli = Cli::parse();
    for path in cli.paths.iter() {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut fmt_txt = FormatedTextFile::new();
        
        for line in reader.lines() {
            fmt_txt.read_line(line.unwrap());
        }
    }
}
