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
    let markers_and_data = file_as_bytes.map(|bytes| convert_bytes_to_markers(&bytes));
}

fn file_to_bytes (path : &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));
        Ok(bytes)
    })
}

// FIXME - should really convert this to an option, propogate errors through correctly
fn convert_bytes_to_markers (bytes : &Vec<u8>) -> Option<Vec<(Marker, Vec<u8>)>> {
    let mut bytes_consumed = 0;
    let mut result = Some(Vec::<(Marker, Vec<u8>)>::new());
    while bytes_consumed < bytes.len() && result.is_some() {
        match get_marker_from_bytes(bytes) {
            None => result = None,  // Failure
            Some(marker) => {
                if marker == Marker::StartOfImage || marker == Marker::EndOfImage {
                    // Special case - these have no length or data segments
                    result.as_mut().unwrap().push((marker, vec![]));
                    bytes_consumed += 2;
                } else {
                    if bytes_consumed + 4 < bytes.len() {
                        result = None;  // Not enough room for the data bytes left, must be duff data
                    } else {
                        // FIXME - still writing this section
                        let data_bytes = &bytes[bytes_consumed + 2 .. bytes_consumed + 4];
                        let data_length = (data_bytes[0] as usize)*256usize + (data_bytes[1] as usize);
                        bytes_consumed += 4;    // Now counts the marker and length bytes
                        let remaining_length = bytes.len() - bytes_consumed;
                        if data_length > remaining_length {
                            result = None;
                        } else {
                            let data = &data_bytes[bytes_consumed .. bytes_consumed + data_length];
                            result.as_mut().unwrap().push((marker, data.to_vec()));
                            bytes_consumed += data_length;
                        }
                    }
                }
            },
        }
    }
    result
}

fn get_marker_from_bytes (bytes : &[u8]) -> Option<Marker> {
    if bytes.len() < 2 {
        None
    } else {
        match Marker::from_bytes(&bytes[0..2]) {
            Ok(marker) => Some(marker),
            Err(_)     => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use marker::*;
    
    #[test]
    fn bytes_to_marker() {
        let bytes1 = [0u8];
        let result1 = Marker::from_bytes(&bytes1);
        if let Err(invalid_marker) = result1 {
            assert_eq!(invalid_marker.description(), "Too few bytes to be a valid marker.")
        } else {
            panic!("Single byte marker did not return an error.")
        }

        let bytes2 = [0u8, 0u8, 0u8];
        let result2 = Marker::from_bytes(&bytes2);
        if let Err(invalid_marker) = result2 {
            assert_eq!(invalid_marker.description(), "Too many bytes to be a valid marker.")
        } else {
            panic!("Three byte marker did not return an error.")
        }

        let bytes3 = [0u8, 0u8];
        let result3 = Marker::from_bytes(&bytes3);
        if let Err(invalid_marker) = result3 {
            assert_eq!(invalid_marker.description(), "00 00 is not a valid marker.")
        } else {
            panic!("Nonsense marker did not return an error.")
        }

        let bytes4 = [0xffu8, 0xd8u8];
        let result4 = Marker::from_bytes(&bytes4);
        if let Ok(marker) = result4 {
            assert_eq!(marker, Marker::StartOfImage);
        } else {
            panic!("Valid marker returned an error.");
        }
    }
}
