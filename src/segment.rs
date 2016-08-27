use marker::*;
use std::error;

pub struct Segment {
    marker : Marker,
    data : Option<Vec<u8>>,
}

impl Segment {
    pub fn read_from_start_of_bytes(bytes : &[u8]) -> Result<Segment, InvalidSegmentError> {
        panic!("Not implemented.");
        //if bytes.len() < 2 {
            //InvalidSegmentError::
    }
}

#[derive(Debug)]
pub struct InvalidSegmentError {
    message : String,
    underlying_error : Option<&error::Error>,
}

impl InvalidSegmentError {
    fn too_few_bytes(n : i8) -> InvalidSegmentError {
        if n == 0 {
            InvalidSegmentError { message : String::from("Attempted to read segment from an empty byte slice."), underlying_error : None }
        } else {
            InvalidSegmentError { message : String::from(format!("Attempted to read segment from a slice containing only {} bytes.", n)), underlying_error : None }
        }
    }
}
