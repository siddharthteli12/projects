use std::fmt::Display;

use crate::Error;

#[derive(Eq, Debug, PartialEq)]
pub struct ChunkType {
    pub val: u32,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            val: u32::from_be_bytes(value),
        })
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // COnfirming that all values are alpha values.
        for c in s.chars() {
            if !c.is_alphabetic() {
                return Err(Error::from("Not allowed to have a non alpha value here"));
            }
        }
        Ok(Self {
            val: u32::from_be_bytes(s.as_bytes().try_into()?),
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = &self.bytes();
        let str = String::from_utf8_lossy(str);
        write!(f, "{}", str)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        u32::to_be_bytes(self.val)
    }

    // `Note` - Not sure if this is correct.
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        is_uppecase(self.bytes()[0])
    }
    fn is_public(&self) -> bool {
        is_uppecase(self.bytes()[1])
    }
    fn is_reserved_bit_valid(&self) -> bool {
        is_uppecase(self.bytes()[2])
    }
    fn is_safe_to_copy(&self) -> bool {
        !is_uppecase(self.bytes()[3])
    }
}

const UPPERCASE_INDEX: u8 = 5;
// Helper method to get any bit value from a `u8`.
// If 5th bit is 0 value is uppercase if its not value is lowercase.
pub fn is_uppecase(value: u8) -> bool {
    (value >> UPPERCASE_INDEX) & 1 == 0
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
