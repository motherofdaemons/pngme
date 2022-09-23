use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;
use std::io::{BufReader, Read};

const MAXIMUM_LENGTH: u32 = (1 << 31) - 1;

struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

// Something went wrong while decoding a chunk.
#[derive(Debug)]
pub struct ChunkDecodingError {
    // The reason that decoding went wrong.
    reason: String,
}
impl ChunkDecodingError {
    fn boxed(reason: String) -> Box<Self> {
        Box::new(Self { reason })
    }
}

impl Display for ChunkDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad chunk: {}", self.reason)
    }
}

impl std::error::Error for ChunkDecodingError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let mut buf: [u8; 4] = [0; 4];
        reader.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        if length > MAXIMUM_LENGTH {
            return Err(ChunkDecodingError::boxed(format!(
                "Length is too long ({} > 2^31 - 1)",
                length
            )));
        }
        reader.read_exact(&mut buf)?;
        let chunk_type = ChunkType::try_from(buf)?;
        let mut chunk_data: Vec<u8> = vec![0; usize::try_from(length)?];
        reader.read_exact(&mut chunk_data)?;
        if chunk_data.len() != length.try_into()? {
            return Err(ChunkDecodingError::boxed(format!(
                "Data is the wrong length {} expected {}",
                chunk_data.len(),
                length
            )));
        }
        reader.read_exact(&mut buf)?;
        let provided_crc = u32::from_be_bytes(buf);
        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(chunk_data.iter())
            .copied()
            .collect();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);
        if provided_crc != crc {
            return Err(ChunkDecodingError::boxed(format!("Bad crc given!")));
        }
        Ok(Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.chunk_type, self.data_as_string().unwrap())
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);
        Self {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc,
        }
    }
    fn length(&self) -> u32 {
        self.length
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone()).map_err(Box::new)?)
    }
    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
