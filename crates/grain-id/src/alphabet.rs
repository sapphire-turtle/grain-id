/// Lookup table for encoding integer to BASE32 characters.
///
/// # Examples
///
/// ```
/// # use grain_id::alphabet::ENCODE_TABLE;
/// assert_eq!(
///     ENCODE_TABLE,
///     &[
///         '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
///         'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
///     ]
/// );
/// ```
pub const ENCODE_TABLE: &[char; 32] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j',
    'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
];

/// A delimiter used in encoding.
///
/// # Examples
///
/// ```
/// # use grain_id::alphabet::ENCODE_DELIMITER;
/// assert_eq!(ENCODE_DELIMITER, '-');
/// ```
pub const ENCODE_DELIMITER: char = '-';

pub(crate) const BASE: u8 = ENCODE_TABLE.len() as u8;

/// Lookup table for decoding BASE32 characters to values
///
/// Maps ASCII character codes(0-255) to their corresponding BASE32 value (0-31).
/// Invalid characters map to 0xFF.
///
/// # Examples
///
/// ```
/// # use grain_id::alphabet::DECODE_TABLE;
/// assert_eq!(
///     DECODE_TABLE,
///     &[
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5,
///         6, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18,
///         19, 1, 20, 21, 0, 22, 23, 24, 25, 26, 27, 27, 28, 29, 30, 31, 255, 255, 255, 255, 255,
///         255, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, 27,
///         27, 28, 29, 30, 31, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
///         255
///     ]
/// );
/// ```
pub const DECODE_TABLE: &[u8; 256] = &{
    let mut buf = [0; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = match i {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'a' => 10,
            b'b' => 11,
            b'c' => 12,
            b'd' => 13,
            b'e' => 14,
            b'f' => 15,
            b'g' => 16,
            b'h' => 17,
            b'i' => 1,
            b'j' => 18,
            b'k' => 19,
            b'l' => 1,
            b'm' => 20,
            b'n' => 21,
            b'o' => 0,
            b'p' => 22,
            b'q' => 23,
            b'r' => 24,
            b's' => 25,
            b't' => 26,
            b'u' => 27,
            b'v' => 27,
            b'w' => 28,
            b'x' => 29,
            b'y' => 30,
            b'z' => 31,
            b'A' => 10,
            b'B' => 11,
            b'C' => 12,
            b'D' => 13,
            b'E' => 14,
            b'F' => 15,
            b'G' => 16,
            b'H' => 17,
            b'I' => 1,
            b'J' => 18,
            b'K' => 19,
            b'L' => 1,
            b'M' => 20,
            b'N' => 21,
            b'O' => 0,
            b'P' => 22,
            b'Q' => 23,
            b'R' => 24,
            b'S' => 25,
            b'T' => 26,
            b'U' => 27,
            b'V' => 27,
            b'W' => 28,
            b'X' => 29,
            b'Y' => 30,
            b'Z' => 31,
            _ => u8::MAX,
        };
        if i == 255 {
            break buf;
        }
        i += 1
    }
};

/// Lookup table for decoding delimiter characters to boolean.
///
/// Valid characters ('-' and '_') map to true and others map to false.
///
/// ```
/// # use grain_id::alphabet::DECODE_DELIMITER_TABLE;
/// assert_eq!(
///     DECODE_DELIMITER_TABLE,
///     &[
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, true, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, true,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false, false, false, false, false, false, false, false, false,
///         false, false, false, false
///     ]
/// );
/// ```
pub const DECODE_DELIMITER_TABLE: &[bool; 256] = &{
    let mut buf = [false; 256];
    let mut i: u8 = 0;

    loop {
        buf[i as usize] = matches!(i, b'-' | b'_');
        if i == 255 {
            break buf;
        }
        i += 1
    }
};

/// Encode 5bit uint to char.
///
/// First 3 bits are ignored.
pub(crate) const fn u5_to_char_lossy(value: u8) -> char {
    let value = value & 0b00011111;
    ENCODE_TABLE[value as usize]
}

/// Decode char to 5bit uint
pub(crate) fn char_to_u5(c: char) -> Option<u8> {
    match TryInto::<u8>::try_into(c) {
        Ok(i) => match DECODE_TABLE[i as usize] {
            u8::MAX => None,
            j => Some(j),
        },
        Err(_) => None,
    }
}
