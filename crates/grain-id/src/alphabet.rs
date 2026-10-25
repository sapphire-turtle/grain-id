use crate::Error;

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

/// Encodes the lower 35 bits of `value` as 7 ASCII characters, most significant first.
pub(crate) const fn encode(value: u64) -> [u8; 7] {
    let mut buf = [0; 7];
    let mut i = 0;
    while i < 7 {
        let u5 = (value >> (5 * (6 - i))) & 0b11111;
        buf[i] = ENCODE_TABLE[u5 as usize] as u8;
        i += 1;
    }
    buf
}

/// Views bytes produced by [`encode`] as a string.
pub(crate) fn as_str(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes).expect("ENCODE_TABLE contains only ASCII characters")
}

/// Decodes up to 7 characters into 5-bit groups, packed from the most significant
/// position of the 35-bit value (so `"a1"` decodes like `"a100000"`).
///
/// Returns the packed value and the number of decoded characters.
pub(crate) fn decode(s: &str) -> Result<(u64, u8), Error> {
    let len = s.chars().count();
    if len > 7 {
        return Err(Error::InvalidLength(len));
    }
    let mut value = 0;
    for (index, character) in s.chars().enumerate() {
        let u5 = char_to_u5(character).ok_or(Error::InvalidCharacter { character, index })?;
        value |= (u5 as u64) << (5 * (6 - index));
    }
    Ok((value, len as u8))
}

/// Decodes a character to a 5-bit value.
fn char_to_u5(c: char) -> Option<u8> {
    let i = u8::try_from(c).ok()?;
    match DECODE_TABLE[i as usize] {
        u8::MAX => None,
        j => Some(j),
    }
}
