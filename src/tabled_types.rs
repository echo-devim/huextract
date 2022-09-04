//! This module defines some glue types for custom display

/// Print hexadecimal format
// pub struct HexValue(HexValueType);

pub enum HexValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

// impl std::fmt::Display for HexValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl std::fmt::Display for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(val) => write!(f, "0x{:x}", val),
            Self::U16(val) => write!(f, "0x{:x}", val),
            Self::U32(val) => write!(f, "0x{:x}", val),
            Self::U64(val) => write!(f, "0x{:x}", val),
            Self::U128(val) => write!(f, "0x{:x}", val),
        }
    }
}

impl From<&[u8]> for HexValue {
    fn from(values: &[u8]) -> Self {
        match values.len() {
            1 => Self::U8(u8::from_le_bytes(
                values.try_into().expect("Incorrect slice len"),
            )),
            2 => Self::U16(u16::from_le_bytes(
                values.try_into().expect("Incorrect slice len"),
            )),
            4 => Self::U32(u32::from_le_bytes(
                values.try_into().expect("Incorrect slice len"),
            )),
            8 => Self::U64(u64::from_le_bytes(
                values.try_into().expect("Incorrect slice len"),
            )),
            16 => Self::U128(u128::from_le_bytes(
                values.try_into().expect("Incorrect slice len"),
            )),
            v => panic!("Unable to create HexValueType: incorrect slice len: {v}"),
        }
    }
}

/// Print array
pub struct ArrayValue(Vec<u8>);

impl From<&[u8]> for ArrayValue {
    fn from(slice: &[u8]) -> Self {
        ArrayValue(Vec::from(slice))
    }
}

impl From<[u8; 1]> for ArrayValue {
    fn from(array: [u8; 1]) -> Self {
        ArrayValue(Vec::from(array.as_slice()))
    }
}

impl From<[u8; 2]> for ArrayValue {
    fn from(array: [u8; 2]) -> Self {
        ArrayValue(Vec::from(array.as_slice()))
    }
}

impl From<[u8; 4]> for ArrayValue {
    fn from(array: [u8; 4]) -> Self {
        ArrayValue(Vec::from(array.as_slice()))
    }
}

impl From<[u8; 8]> for ArrayValue {
    fn from(array: [u8; 8]) -> Self {
        ArrayValue(Vec::from(array.as_slice()))
    }
}

impl From<[u8; 16]> for ArrayValue {
    fn from(array: [u8; 16]) -> Self {
        ArrayValue(Vec::from(array.as_slice()))
    }
}

impl std::fmt::Display for ArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values_per_line = match self.0.len() {
            0..=8 => 2,
            _ => 4,
        };
        let mut display = String::new();
        for (i, val) in self.0.iter().enumerate() {
            if i == 0 {
                display.push_str(format!("0x{:02x}", val).as_str());
            } else if i % values_per_line == 0 {
                display.push_str(format!("\n0x{:02x}", val).as_str());
            } else {
                display.push_str(format!(" 0x{:02x}", val).as_str());
            }
        }
        write!(f, "{}", display)
    }
}
