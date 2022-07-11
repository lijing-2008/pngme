use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::Result;

use crate::error::PngError;

/**
fn bytes(&self) -> [u8; 4]
fn is_valid(&self) -> bool
fn is_critical(&self) -> bool
fn is_public(&self) -> bool
fn is_reserved_bit_valid(&self) -> bool
fn is_safe_to_copy(&self) -> bool
 */
#[derive(Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    // bit 5 of first byte
    // 0-uppercase-critical
    // 1-lowercase-ancillary
    fn is_critical(&self) -> bool {
        self.0.is_ascii_uppercase()
    }

    // bit 5 of second byte
    // 0-uppercase-public
    // 1-lowercase-private
    fn is_public(&self) -> bool {
        self.1.is_ascii_uppercase()
    }

    // bit 5 of third byte
    // must be 0-uppercase
    fn is_reserved_bit_valid(&self) -> bool {
        self.2.is_ascii_uppercase()
    }

    // bit 5 of second byte
    // 0-uppercase-unsafe to copy
    // 1-lowercase-safe to copy
    fn is_safe_to_copy(&self) -> bool {
        self.3.is_ascii_lowercase()
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_err(&self) -> bool {
        !(self.0.is_ascii_alphabetic()
            && self.1.is_ascii_alphabetic()
            && self.2.is_ascii_alphabetic()
            && self.3.is_ascii_alphabetic())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngError;

    fn try_from(value: [u8; 4]) -> Result<Self, PngError> {
        for val in value {
            match val.is_ascii_alphabetic() {
                true => {}
                _ => return Err(PngError::ChunkTypeError)
            }
        }
        Ok(ChunkType(value[0], value[1], value[2], value[3]))
    }
}

impl FromStr for ChunkType {
    type Err = PngError;

    fn from_str(s: &str) -> std::result::Result<Self, PngError> {
        let value = s.as_bytes();
        ChunkType::try_from(<[u8; 4]>::try_from(value).unwrap())
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.bytes().to_vec()).unwrap();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let a = 82;
        let b = 114;
        println!("{:b}", a);
        println!("{:b}", b);
    }

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
