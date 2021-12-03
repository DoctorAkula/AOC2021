use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn loadinput(filename : &str) -> Vec<String> {
    let source_file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("cannot open file: {}", e.to_string());
            exit(-1);
        }
    };

    let reader = BufReader::new(source_file);
    reader.lines().map(|l| l.unwrap()).collect()
}
