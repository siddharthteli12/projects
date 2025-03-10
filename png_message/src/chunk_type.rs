use crate::Error;
use std::{
    fmt::{write, Display},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    type_code: u32,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType {
            type_code: u32::from_be_bytes(value),
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes().try_into()?;

        // Validate the bytes that they are alpha only.
        for val in bytes {
            if !val.is_ascii_alphabetic() {
                return Err(Error::from("Only alphas are allowed"));
            }
        }

        println!("{:?}", bytes);
        let result = ChunkType {
            type_code: u32::from_be_bytes(bytes),
        };
        println!("{:?}", result);
        Ok(result)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.type_code.to_be_bytes();
        write!(f, "{}", String::from_utf8(output.into()).unwrap())
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.type_code.to_be_bytes()
    }

    fn is_valid(&self) -> bool {
        let asci_bytes = self.bytes();
        println!("{:?}", asci_bytes);
        // Validate for each char
        for val in asci_bytes {
            if !val.is_ascii_alphabetic() {
                return false;
            }
        }

        // Validate third char is uppercase.
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        if self.bytes()[0].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }

    fn is_public(&self) -> bool {
        if self.bytes()[1].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }

    fn is_reserved_bit_valid(&self) -> bool {
        if self.bytes()[2].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }

    fn is_safe_to_copy(&self) -> bool {
        if self.bytes()[3].is_ascii_lowercase() {
            true
        } else {
            false
        }
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
