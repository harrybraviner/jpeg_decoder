#![feature(box_syntax)]
#![feature(slice_patterns)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod marker;
mod segment;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use marker::*;
use segment::*;
use std::error::Error;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Too few arguments!");
    } else if args.len() > 3 {
        panic!("Too many arguments!");
    }

    let input_filename = &args[1];
    let file_as_bytes = file_to_bytes(Path::new(input_filename)).unwrap();

    let segments = Segment::parse_bytes_to_segments(&file_as_bytes);
    match segments {
        Err(error) => println!("{}", format!("Failed to parse the file to segments.\nGot error: {}", error.description())),
        Ok(segments) => {
            print!("[");
            for segment in segments {
                print!("{}, ", segment.summary_string());
            }
            println!("]");
        }
    }
}

fn file_to_bytes (path : &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));
        Ok(bytes)
    })
}
