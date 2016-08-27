use std::fmt;
use std::error;

#[derive(Debug, PartialEq)]
pub enum Marker {
    DefineHuffmanTable,
    StartOfImage,
    EndOfImage,
    StartOfScan,
    DefineQuantizationTable,
    Comment,
}

impl Marker {
    pub fn from_bytes(bytes : &[u8]) -> Result<Marker, InvalidMarkerError> {
        if bytes.len() != 2 {
            Err(InvalidMarkerError::new(bytes))
        } else {
            match bytes {
                &[0xffu8, 0xc4u8] => Ok(Marker::DefineHuffmanTable),
                &[0xffu8, 0xd8u8] => Ok(Marker::StartOfImage),
                &[0xffu8, 0xd9u8] => Ok(Marker::EndOfImage),
                &[0xffu8, 0xdau8] => Ok(Marker::StartOfScan),
                &[0xffu8, 0xdbu8] => Ok(Marker::DefineQuantizationTable),
                &[0xffu8, 0xfeu8] => Ok(Marker::Comment),
                _ => Err(InvalidMarkerError::new(bytes)),
            }
        }
    }
}

#[derive(Debug)]
pub struct InvalidMarkerError {
    bytes : Vec<u8>,
    message : String,
}

impl InvalidMarkerError {
    fn new (bytes : &[u8]) -> InvalidMarkerError {
        let message =
            if bytes.len() > 2 {
                String::from("Too many bytes to be a valid marker.")
            } else if bytes.len() < 2 {
                String::from("Too few bytes to be a valid marker.")
            } else {
                format!("{:02x} {:02x} is not a valid marker.", bytes[0], bytes[1])
            };
        InvalidMarkerError { bytes : Vec::from(bytes), message : message }
    }
}

impl fmt::Display for InvalidMarkerError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid marker")
    }
}

impl error::Error for InvalidMarkerError {
    fn description(&self) -> &str {
        &self.message[..]
    }
}


