use marker::*;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Segment {
    marker : Marker,
    data : Option<Vec<u8>>,
}

impl Segment {
    pub fn read_from_start_of_bytes(bytes : &[u8]) -> Result<Segment, InvalidSegmentError> {
        let bytes_len = bytes.len();
        if bytes_len < 2 {
            Err(InvalidSegmentError::too_few_bytes(bytes_len))
        } else {
            let marker = Marker::from_bytes(&bytes[0..2]);
            match marker {
                Err(error) => Err(InvalidSegmentError::invalid_marker(error)),
                Ok(marker) => {
                    if marker == Marker::StartOfImage || marker == Marker::EndOfImage {
                        // Easy case - these markers have no length or data
                        Ok(Segment { marker : marker, data : None })
                    } else {
                        if bytes_len < 4 {
                            Err(InvalidSegmentError::no_length_bytes())
                        } else {
                            let length = (bytes[2] as usize)*256 + (bytes[3] as usize);
                            if length < 2 {
                                Err(InvalidSegmentError::length_less_than_two(length))
                            } else if bytes_len < length + 2 {
                                Err(InvalidSegmentError::too_few_data_bytes(length - 2, bytes_len - 4))
                            } else {
                                Ok(Segment { marker : marker, data : Some(bytes[4..4+length-2].to_vec())})
                            }
                        }
                    }
                }
            }
        }
            
    }
}

#[derive(Debug)]
pub struct InvalidSegmentError {
    pub message : String,
    underlying_error : Option<Box<Error>>,
}

impl InvalidSegmentError {
    fn too_few_bytes(n : usize) -> InvalidSegmentError {
        if n == 0 {
            InvalidSegmentError { message : String::from("Attempted to read segment from an empty byte slice."), underlying_error : None }
        } else {
            InvalidSegmentError { message : String::from(format!("Attempted to read segment from a slice containing only {} bytes.", n)), underlying_error : None }
        }
    }

    fn invalid_marker(error : InvalidMarkerError) -> InvalidSegmentError {
        InvalidSegmentError { message : String::from("Segment began with an invalid marker."), underlying_error : Some(box(error)) }
    }

    fn no_length_bytes() -> InvalidSegmentError {
        InvalidSegmentError { message : String::from("Marker requires length bytes, but have fewer than two byes left in the input."), underlying_error : None }
    }

    fn length_less_than_two(n : usize) -> InvalidSegmentError {
        InvalidSegmentError { message : String::from(format!("Length of segment, {}, was less than two. This doesn't even cover the two data bytes!", n)), underlying_error : None }
    }

    fn too_few_data_bytes(n_expected : usize, n_actual : usize) -> InvalidSegmentError {
        InvalidSegmentError {message : String::from(format!("Segment wants {} data bytes, but there are only {} bytes remaining in the slice.", n_expected, n_actual)), underlying_error : None }
    }
}

impl fmt::Display for InvalidSegmentError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid segment")
    }
}

impl Error for InvalidSegmentError {
    fn description(&self) -> &str {
        &self.message[..]
    }

    fn cause(&self) -> Option<&Error> {
        self.underlying_error.as_ref().map(|boxed_error| &**boxed_error)
    }
}

#[cfg(test)]
mod segment_tests {
    use marker::*;
    use super::*;
    use std::error::Error;

    #[test]
    fn too_few_bytes() {
        let bytes : Vec<u8> = vec![];
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_err = InvalidSegmentError::too_few_bytes(0);

        assert!(result.is_err());

        assert_eq!(result.unwrap_err().message, expected_err.message);

        let bytes1 : Vec<u8> = vec![0u8];
        let result1 = Segment::read_from_start_of_bytes(&bytes1[..]);
        let expected_err1 = InvalidSegmentError::too_few_bytes(1);

        assert!(result1.is_err());

        assert_eq!(result1.unwrap_err().message, expected_err1.message);
    }

    #[test]
    fn invalid_marker() {
        let bytes = vec![0u8, 0u8];
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_inner_error = Marker::from_bytes(&bytes[..]).unwrap_err();
        let expected_inner_description = String::from(expected_inner_error.description());
        let expected_err = InvalidSegmentError::invalid_marker(expected_inner_error);

        assert!(result.is_err());

        assert_eq!(result.as_ref().unwrap_err().message, expected_err.message);
        assert_eq!((*result.unwrap_err().underlying_error.unwrap()).description(), expected_inner_description);
    }

    #[test]
    fn no_length_bytes() {
        let bytes = vec![0xffu8, 0xfeu8];   // Comment marker, should have length bytes
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_err = InvalidSegmentError::no_length_bytes();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, expected_err.message);

        let bytes1 = vec![0xffu8, 0xfeu8, 0x01u8];   // Comment marker, should have length bytes
        let result1 = Segment::read_from_start_of_bytes(&bytes1[..]);
        let expected_err1 = InvalidSegmentError::no_length_bytes();

        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err().message, expected_err1.message);
    }


    #[test]
    fn length_less_than_two() {
        let bytes = vec![0xffu8, 0xfeu8, 0x00u8, 0x01u8];   // Comment marker
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_err = InvalidSegmentError::length_less_than_two(1);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, expected_err.message);
    }

    #[test]
    fn too_few_data_bytes() {
        let bytes = vec![0xffu8, 0xfeu8, 0x00u8, 0x06u8, 0xabu8, 0xcdu8, 0xefu8];   // Comment marker, with not enough data
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_err = InvalidSegmentError::too_few_data_bytes(4, 3);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, expected_err.message);
    }


    #[test]
    fn marker_no_requiring_data() {
        let bytes = vec![0xffu8, 0xd8u8, 0x00u8, 0x06u8, 0xabu8, 0xcdu8, 0xefu8];   // StartOfImage, plus padding
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_ok = Segment { marker : Marker::StartOfImage, data : None };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_ok);
    }

    #[test]
    fn marker_with_data() {
        let bytes = vec![0xffu8, 0xfeu8, 0x00u8, 0x06u8, 0xabu8, 0xcdu8, 0xefu8, 0x03u8];   // Comment marker, with data
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_ok = Segment { marker : Marker::Comment, data : Some(vec![0xabu8, 0xcdu8, 0xefu8, 0x03u8]) };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_ok);
    }


    #[test]
    fn marker_with_data_and_padding() {
        let bytes = vec![0xffu8, 0xfeu8, 0x00u8, 0x06u8, 0xabu8, 0xcdu8, 0xefu8, 0x03u8, 0x00u8, 0x17u8];   // Comment marker, with data
        let result = Segment::read_from_start_of_bytes(&bytes[..]);
        let expected_ok = Segment { marker : Marker::Comment, data : Some(vec![0xabu8, 0xcdu8, 0xefu8, 0x03u8]) };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_ok);
    }
}
