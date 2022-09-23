use crate::{MyError, Result};
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum ChunkTypeError {
    ByteLengthError(usize),
    InvalidCharacter,
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError(length) => write!(
                f,
                "Expected 4 bytes but recieved {} when creating chunk type",
                length
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Input contains one or more invalid characters!")
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    raw_bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.raw_bytes
    }
    #[cfg(test)]
    fn is_valid(&self) -> bool {
        // all bits must be between a-z or A-Z and the reserved bit but be valid
        self.raw_bytes.iter().all(|&b| b.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }
    #[cfg(test)]
    fn is_critical(&self) -> bool {
        // check to see if the ancillary bit is not set
        self.raw_bytes[0] & (1 << 5) == 0
    }
    #[cfg(test)]
    fn is_public(&self) -> bool {
        // check to see if the private bit is not set
        self.raw_bytes[1] & (1 << 5) == 0
    }
    #[cfg(test)]
    fn is_reserved_bit_valid(&self) -> bool {
        // check to see if the reserved bit is not set
        self.raw_bytes[2] & (1 << 5) == 0
    }
    #[cfg(test)]
    fn is_safe_to_copy(&self) -> bool {
        // check to see if the safe to copy bit is set
        self.raw_bytes[3] & (1 << 5) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = MyError;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(Self { raw_bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())));
        }
        let valid_chars = bytes.iter().all(|&b| b.is_ascii_alphabetic());
        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }
        Ok(Self { raw_bytes: bytes.try_into()? })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.raw_bytes).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

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
