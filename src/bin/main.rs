use binlang::{compile, run, BLError};
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: binlang <srcfile>");
        return;
    }
    let path = Path::new(&args[1]);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(s) => {
            println!("could not open file: {}", s);
            return;
        }
    };
    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Ok(_) => {},
        Err(s) => {
            println!("could not read file: {}", s);
            return;
        }
    };

    //let src = "let a = 1;\nlet b = {1,0,1,1};\nlet c = {:4};\nlet d = {0,1,};\nlet _a = 1;\nlet b_ = 0;";
    let prg = match compile(&src) {
        Ok(p) => p,
        Err(e) => {
            println!("There was a problem parsing the program.");
            match e {
                BLError::GeneralError => { println!("(general error)"); },
                BLError::ParserError(s) => { println!(">\t{}", s); },
            }
            return;
        }
    };
    run(prg).unwrap();
}
