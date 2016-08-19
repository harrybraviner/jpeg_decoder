#![feature(slice_patterns)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error;
use std::fmt;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Too few arguments!");
    } else if args.len() > 3 {
        panic!("Too many arguments!");
    }

    let input_filename = &args[1];
    let file_as_bytes = file_to_bytes(Path::new(input_filename));

    // FIXME - implement loop over bytes
}

fn file_to_bytes (path : &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));
        Ok(bytes)
    })
}

#[derive(Debug, PartialEq)]
enum Marker {
    DefineHuffmanTable,
    StartOfImage,
    EndOfImage,
    StartOfScan,
    DefineQuantizationTable,
    Comment,
}

#[derive(Debug)]
struct InvalidMarker {
    bytes : Vec<u8>,
    message : String,
}

impl InvalidMarker {
    fn new (bytes : &[u8]) -> InvalidMarker {
        let message =
            if bytes.len() > 2 {
                String::from("Too many bytes to be a valid marker.")
            } else if bytes.len() < 2 {
                String::from("Too few bytes to be a valid marker.")
            } else {
                format!("{:02x} {:02x} is not a valid marker.", bytes[0], bytes[1])
            };
        InvalidMarker { bytes : Vec::from(bytes), message : message }
    }
}

impl fmt::Display for InvalidMarker {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid marker")
    }
}

impl error::Error for InvalidMarker {
    fn description(&self) -> &str {
        &self.message[..]
    }
}

fn get_marker_from_bytes (bytes : &[u8]) -> Option<Marker> {
    if bytes.len() < 2 {
        None
    } else {
        match bytes_to_marker(&bytes[0..2]) {
            Ok(marker) => Some(marker),
            Err(_)     => None,
        }
    }
}

fn bytes_to_marker (bytes : &[u8]) -> Result<Marker, InvalidMarker> {
    if bytes.len() != 2 {
        Err(InvalidMarker::new(bytes))
    } else {
        match bytes {
            &[0xffu8, 0xc4u8] => Ok(Marker::DefineHuffmanTable),
            &[0xffu8, 0xd8u8] => Ok(Marker::StartOfImage),
            &[0xffu8, 0xd9u8] => Ok(Marker::EndOfImage),
            &[0xffu8, 0xdau8] => Ok(Marker::StartOfScan),
            &[0xffu8, 0xdbu8] => Ok(Marker::DefineQuantizationTable),
            &[0xffu8, 0xfeu8] => Ok(Marker::Comment),
            _ => Err(InvalidMarker::new(bytes)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    
    #[test]
    fn bytes_to_marker() {
        let bytes1 = [0u8];
        let result1 = super::bytes_to_marker(&bytes1);
        if let Err(invalid_marker) = result1 {
            assert_eq!(invalid_marker.description(), "Too few bytes to be a valid marker.")
        } else {
            panic!("Single byte marker did not return an error.")
        }

        let bytes2 = [0u8, 0u8, 0u8];
        let result2 = super::bytes_to_marker(&bytes2);
        if let Err(invalid_marker) = result2 {
            assert_eq!(invalid_marker.description(), "Too many bytes to be a valid marker.")
        } else {
            panic!("Three byte marker did not return an error.")
        }

        let bytes3 = [0u8, 0u8];
        let result3 = super::bytes_to_marker(&bytes3);
        if let Err(invalid_marker) = result3 {
            assert_eq!(invalid_marker.description(), "00 00 is not a valid marker.")
        } else {
            panic!("Nonsense marker did not return an error.")
        }

        let bytes4 = [0xffu8, 0xd8u8];
        let result4 = super::bytes_to_marker(&bytes4);
        if let Ok(marker) = result4 {
            assert_eq!(marker, super::Marker::StartOfImage);
        } else {
            panic!("Valid marker returned an error.");
        }
    }
}
