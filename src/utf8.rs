/*!
UTF-8 encoding and decoding.

# Encoding
A unicode code point is represented using one to four bytes in UTF-8, depending on its value.
* If the unicode code point is in the range `0x0000` to `0x007F`, it is represented using [one byte](#one-byte).
* If the unicode code point is in the range `0x0080` to `0x07FF`, it is represented using [two bytes](#two-bytes).
* If the unicode code point is in the range `0x0800` to `0xFFFF`, it is represented using [three bytes](#three-bytes).
* If the unicode code point is in the range `0x10000` to `0x10FFFF`, it is represented using [four bytes](#four-bytes).

# Decoding
A UTF-8 code point is decoded into a unicode code point using the following rules:
* If the first bit of the UTF-8 code point is 0, the unicode code point is represented using [one byte](#one-byte).
* If the first three bits of the UTF-8 code point are 110, the unicode code point is represented using [two bytes](#two-bytes).
* If the first four bits of the UTF-8 code point are 1110, the unicode code point is represented using [three bytes](#three-bytes).
* If the first five bits of the UTF-8 code point are 11110, the unicode code point is represented using [four bytes](#four-bytes).

When a unicode code point is represented using two, three or four bytes, these bytes of the UTF-8 code point are continuation bytes. The continuation bytes start with the bit pattern `10`.

## Representation

**Note**:

* UTF-8 is a prefix code, which means that no UTF-8 code point is a prefix of another UTF-8 code point. This means that the first byte of a UTF-8 code point is enough to determine the length of the UTF-8 code point and decode it in an unambiguous way.
* UTF-8 is capable of encoding all 1,112,064 valid unicode code points in Unicode.
* The number of `x`s on the right side of the `0` in the unicode code point are the number of free bits on the UTF-8 code point.

### One byte

**Encoding**: If the eighth bit of the unicode code point is 0, the unicode code point is represented in UTF-8 using one byte.

**Decoding**: If the UTF-8 code point starts with a 0, the unicode code point is represented using only the first eight least significant bits.

* Unicode code point: `nnnnnnnn|nnnnnnnn|nnnnnnnn|0xxxxxxx`
* UTF-8 code point: `0xxxxxxx`

### Two bytes

**Encoding**: If the twelfth bit of the unicode code point is 0, the unicode code point is represented in UTF-8 using two bytes.

**Decoding**: If the UTF-8 code point starts with `110` it has only one continuation byte, and the unicode code point is represented using the first eleven least significant bits (excluding the prefix bits).

* Unicode code point: `nnnnnnnn|nnnnnnnn|nnnn0xxx|xxxxxxxx`
* UTF-8 code point: `110xxxxx|10xxxxxx`

### Three bytes

**Encoding**: If the seventeenth bit of the unicode code point is 0, the unicode code point is represented using three bytes.

**Decoding**: If the UTF-8 code point starts with `1110` it has two continuation bytes, and the unicode code point is represented using the first sixteen least significant bits (excluding the prefix bits).

* Unicode code point: `nnnnnnnn|nnnnnnn0|xxxxxxxx|xxxxxxxx`
* UTF-8 code point: `1110xxxx|10xxxxxx|10xxxxxx`

### Four bytes

**Encoding**:  If the twentysecond bit of the unicode code point is 0, the unicode code point is represented using four bytes.

**Decoding**: If the UTF-8 code point starts with `11110` it has three continuation bytes, and the unicode code point is represented using the first twenty-one least significant bits (excluding the prefix bits).

* Unicode code point: `nnnnnnnn|nn0xxxxx|xxxxxxxx|xxxxxxxx`
* UTF-8 code point: `11110xxx|10xxxxxx|10xxxxxx|10xxxxxx`
*/

// use crate::prelude::*;
// use crate::unicode::check_code_point;
use crate::unicode;

/// Encode a unicode code point into a vector of UTF-8 code points.
///
/// # Parameters
/// * `code_point`: [`u32`] - A unicode code point.
///
/// # Returns
/// A [`Vec<u8>`] containing the UTF-8 code points.
///
/// # Panics
/// * If the input unicode code point is invalid.
fn encode_code_point(unicode_cp: u32) -> Vec<u8> {
    if (unicode_cp & 0xFFFFFF80) == 0 {
        return vec![unicode_cp as u8];
    }

    let mut byte_vec: Vec<u8> = Vec::new();
    if (unicode_cp & 0xFFFFF800) == 0 {
        // unicode_cp: 0x418 -> 0b0000_0100_0001_1000
        // 0x1F -> 0b0001_1111
        //              ^^^^^^__ To be sure that the result is 5 bits
        // 0xC0 -> 0b1100_0000
        //           ^^^________ The start of the two bytes representation
        //
        // 0b0000_0100_0001_1000 >> 6 -> 0b0001_0000
        // 0b0001_0000 & 0b0001_1111 -> 0b0001_0000
        // 0b0001_0000 | 0b1100_0000 -> 0b1101_0000
        byte_vec.push((((unicode_cp >> 6) & 0x1F) | 0xC0) as u8);
    } else if (unicode_cp & 0xFFFF0000) == 0 {
        unicode::check_code_point(unicode_cp);
        // unicode_cp: 0x20AC -> 0b0010_0000_1010_1100
        // 0x0F -> 0b0000_1111
        //                ^^^^__ To be sure that the result is 4 bits
        // 0xE0 -> 0b1110_0000
        //           ^^^^_______ The start of the three bytes representation
        // 0x3F -> 0b0011_1111
        //             ^^^^^^^__ To be sure that the result is 6 bits
        // 0x80 -> 0b1000_0000
        //           ^^_________ The start of the continuation bytes
        //
        // 0b0010_0000_1010_1100 >> 12 -> 0b0000_0010
        // 0b0000_0010 & 0b0000_1111 -> 0b0000_0010
        // 0b0000_0010 | 0b1110_0000 -> 0b1110_0010
        byte_vec.push((((unicode_cp >> 12) & 0x0F) | 0xE0) as u8);
        byte_vec.push((((unicode_cp >> 6) & 0x3F) | 0x80) as u8); // Look at the comment above
    } else if (unicode_cp & 0xFFE00000) == 0 {
        // unicode_cp: 0x10348 -> 0b0001_0000_0011_0100_1000
        // 0x07 -> 0b0000_0111
        //                 ^^^__ To be sure that the result is 3 bits
        // 0xF0 -> 0b1111_0000
        //           ^^^^^^_____ The start of the four bytes representation
        //
        // 0b0001_0000_0011_0100_1000 >> 18 -> 0b0
        // 0b0 & 0b0000_0111 -> 0b0
        // 0b0 | 0b1111_0000 -> 0b1111_0000
        byte_vec.push((((unicode_cp >> 18) & 0x07) | 0xF0) as u8);
        byte_vec.push((((unicode_cp >> 12) & 0x3F) | 0x80) as u8); // Look at the comment above
        byte_vec.push((((unicode_cp >> 6) & 0x3F) | 0x80) as u8); // Look at the comment above
    }
    // unicode_cp: 0x07FF -> 0b0000_0111_1111_1111
    // 0x3F -> 0b0011_1111
    //             ^^^^^^^__ To be sure that the result is 6 bits
    // 0x80 -> 0b1000_0000
    //           ^^_________ The start of the continuation bytes
    //
    // 0b0000_0111_1111_1111 & 0b0011_1111 -> 0b0011_1111
    // 0b0011_1111 | 0b1000_0000 -> 0b1011_1111
    byte_vec.push(((unicode_cp & 0x3F) | 0x80) as u8);
    byte_vec
}

/// Read the next byte from a vector of UTF-8 code points.
///
/// # Parameters
/// * `byte_vec`: [`&[u8]`] - A slice of UTF-8 code points.
/// * `i`: [`usize`] - The index of the byte to read.
///
/// # Returns
/// The next byte from the vector of UTF-8 code points.
///
/// # Panics
/// * If the index `i` is out of bounds.
/// * If the byte at index `i` is not a continuation byte.
fn read_next_byte(byte_vec: &[u8], i: usize) -> u32 {
    if i >= byte_vec.len() {
        panic!("Index out of bounds");
    }
    #[allow(clippy::identity_op)]
    let continuation_byte: u8 = byte_vec[i] & 0xFF;
    // continuation_byte: 0b1011_1111
    // 0xC0 -> 0b1100_0000
    //           ^^_________ In order to compare (keep) the first two bits
    // 0x80 -> 0b1000_0000
    //           ^^_________ The start of the continuation bytes
    // 0x3F -> 0b0011_1111
    //             ^^^^^^^__ To be sure that the result is 6 bits
    //
    //
    // 0b1011_1111 & 0b1100_0000 -> 0b1000_0000
    if (continuation_byte & 0xC0) == 0x80 {
        return (continuation_byte & 0x3F) as u32;
    }
    panic!("Invalid continuation byte");
}

/// Decode a UTF-8 code point into a unicode code point.
///
/// # Parameters
/// * `utf8_cp`: [`&[u8]`] - A slice of UTF-8 code points.
/// * `i`: [`usize`] - The index of the byte to read. It should be the index of a prefix byte.
///
/// # Returns
/// A tuple containing the unicode code point and the number of bytes read.
/// * The unicode code point is the decoded UTF-8 code point.
/// * The number of bytes read is the number of consumed bytes from the vector of UTF-8 code points.
///
/// # Panics
/// * If the index `i` is out of bounds.
/// * If, after reading the first byte, the continuation bytes are not valid.
/// * If the UTF-8 code point is invalid.
fn decode_symbol(utf8_cp: &[u8], i: usize) -> Option<(u32, usize)> {
    if i > utf8_cp.len() {
        panic!("Index out of bounds");
    }

    if i == utf8_cp.len() {
        return None;
    }

    let mut code_point: u32;
    let mut offset: usize = 0;

    #[allow(clippy::identity_op)]
    let byte1: u32 = (utf8_cp[i] & 0xFF) as u32;
    code_point = byte1;
    offset += 1;
    if (byte1 & 0x80) == 0 {
        return Some((code_point, offset));
    }

    if (byte1 & 0xE0) == 0xC0 {
        // utf8_cp: 0xD0, 0x98 -> 0b1101_0000, 0b1001_1000
        // 0xE0 -> 0b1110_0000
        //           ^^^^_______ In order to compare (keep) the first three bits
        // 0xC0 -> 0b1100_0000
        //           ^^^________ The start of the two bytes representation
        // 0x1F -> 0b0001_1111
        //              ^^^^^^__ To be sure that the result is 5 bits
        //
        // 0b1101_0000 & 0b0001_1111 -> 0b0001_0000
        // 0b0001_0000 << 6 -> 0b0000_0100_0000_0000
        //
        // 0b0001_1000
        // ^^^^^^^^^^^__ From reading the next byte
        // 0b0000_0100_0000_0000 |
        //           0b1001_1000 ->
        // 0b0000_0100_0001_1000
        let byte2: u32 = read_next_byte(utf8_cp, i + offset);
        code_point = ((byte1 & 0x1F) << 6) | byte2;
        offset += 1;
        if code_point >= 0x80 {
            return Some((code_point, offset));
        } else {
            panic!("Invalid continuation byte");
        }
    }

    if (byte1 & 0xF0) == 0xE0 {
        // utf8_cp: 0xE2, 0x82, 0xAC -> 0b1110_0010, 0b1000_0010, 0b1010_1100
        // 0xF0 -> 0b1111_0000
        //           ^^^^^______ In order to compare (keep) the first four bits
        // 0xE0 -> 0b1110_0000
        //           ^^^^_______ The start of the three bytes representation
        // 0x0F -> 0b0000_1111
        //                ^^^^__ To be sure that the result is 4 bits
        //
        // 0b1110_0010 & 0b0000_1111 -> 0b0000_0010
        // 0b0000_0010 << 12 -> 0b0010_0000_0000_0000
        //
        // 0b0000_0010 << 6 -> 0b0000_0000_1000_0000
        // ^^^^^^^^^^^__ From reading the next byte
        //
        // 0b0010_1100
        // ^^^^^^^^^^^__ From reading the next byte
        //
        // 0b0010_0000_0000_0000 |
        // 0b0000_0000_1000_0000 |
        //           0b0010_1100 ->
        // 0b0010_0000_1010_1100
        let byte2: u32 = read_next_byte(utf8_cp, i + offset);
        offset += 1;
        let byte3: u32 = read_next_byte(utf8_cp, i + offset);
        offset += 1;
        code_point = ((byte1 & 0x0F) << 12) | (byte2 << 6) | byte3;
        if code_point >= 0x0800 {
            unicode::check_code_point(code_point);
            return Some((code_point, offset));
        } else {
            panic!("Invalid continuation byte");
        }
    }

    if (byte1 & 0xF8) == 0xF0 {
        // utf8_cp: 0xF0, 0x10, 0xD, 0x8 -> 0b1111_0000, 0b0001_0000, 0b0000_1101, 0b0000_1000
        // 0xF8 -> 0b1111_1000
        //           ^^^^^^_____ In order to compare (keep) the first five bits
        //
        // 0xF0 -> 0b1111_0000
        //           ^^^^^______ The start of the four bytes representation
        // 0x07 -> 0b0000_0111
        //                 ^^^__ To be sure that the result is 3 bits
        //
        // 0b1111_0000 & 0b0000_0111 -> 0b0000_0000
        // 0b0000_0000 << 18 -> 0b0000_0000_0000_0000_0000_0000
        //
        // 0b0001_0000 << 12 -> 0b0001_0000_0000_0000_0000
        // ^^^^^^^^^^^__ From reading the next byte
        // 0b0000_1101 << 6 -> 0b0000_0011_0100_0000
        // ^^^^^^^^^^^__ From reading the next byte
        // 0b0000_1000
        // ^^^^^^^^^^^__ From reading the next byte
        //
        // 0b0000_0000_0000_0000_0000_0000 |
        //      0b0001_0000_0000_0000_0000 |
        //           0b0000_0011_0100_0000 |
        //                     0b0000_1000 ->
        // 0b0000_0001_0000_0011_0100_1000
        let byte2: u32 = read_next_byte(utf8_cp, i + offset);
        offset += 1;
        let byte3: u32 = read_next_byte(utf8_cp, i + offset);
        offset += 1;
        let byte4: u32 = read_next_byte(utf8_cp, i + offset);
        offset += 1;
        code_point = ((byte1 & 0x07) << 18) | (byte2 << 12) | (byte3 << 6) | byte4;
        if (0x010000..=0x10FFFF).contains(&code_point) {
            return Some((code_point, offset));
        }
    }

    panic!("Invalid UTF-8 sequence");
}

/// Pretty print the UTF-8 code points in hexadecimal, (binary) and decimal.
///
/// # Parameters
/// * `utf8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the UTF-8 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-8.
fn print_utf8_vec<T: AsRef<Vec<u8>>>(utf8_cp: T, binary_flag: bool) {
    let v: Vec<u8> = utf8_cp.as_ref().to_vec();
    let string_repr: String = String::from_utf8(v.clone()).unwrap();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    println!();
    println!(
        "--------------- UTF-8 of \"{}\" ---------------",
        string_repr
    );
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!(
        "{}{}",
        "-".repeat(43),
        "-".repeat(string_repr.chars().count())
    );
    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

/// Pretty print the UTF-8 encoding in hexadecimal and decimal of a vector of UTF-8 code points.
///
/// # Parameters
/// * `uft8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-8.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81];
/// print_utf8(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-8 encoding of "𐀁" ---------------
/// Hex: [f0, 90, 80, 81]
/// Dec: [240, 144, 128, 129]
/// ----------------------------------------------------
pub fn print_utf8<T: AsRef<Vec<u8>>>(uft8_cp: T) {
    print_utf8_vec(uft8_cp, false);
}

/// Pretty print the UTF-8 encoding in hexadecimal, binary and decimal of a vector of UTF-8 code points.
///
/// # Parameters
/// * `uft8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-8.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81];
/// print_utf8_b(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-8 encoding of "𐀁" ---------------
/// Hex: [f0, 90, 80, 81]
/// Bin: ["11110000", "10010000", "10000000", "10000001"]
/// Dec: [240, 144, 128, 129]
/// -----------------------------------------------------
pub fn print_utf8_b<T: AsRef<Vec<u8>>>(uft8_cp: T) {
    print_utf8_vec(uft8_cp, true);
}

/// Encode a vector of unicode code points into a vector of UTF-8 code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Returns
/// A [`Vec<u8>`] containing the UTF-8 code points.
///
/// # Panics
/// * If the input vector (`unicode_cp`) of unicode code points contains invalid unicode code points.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u32> = vec![0x10001]; // Array of code points in unicode
/// let enc: Vec<u8> = encode_in_utf8(&v);
/// assert_eq!(enc, vec![0xf0, 0x90, 0x80, 0x81]);
/// ```
pub fn encode_in_utf8<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u8> {
    let unicode_cp: Vec<u32> = unicode_cp.as_ref().to_vec();
    let len: usize = unicode_cp.len();
    let mut utf8_cp: Vec<u8> = Vec::new();
    for cp in unicode_cp.iter().take(len) {
        utf8_cp.append(&mut encode_code_point(*cp));
    }
    utf8_cp
}

/// Decode a vector of UTF-8 code points into a vector of unicode code points.
///
/// # Parameters
/// * `utf8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Returns
/// A [`Vec<u32>`] containing the unicode code points.
///
/// # Panics
/// * If the input vector (`utf8_cp`) of UTF-8 code points contains invalid code points.
/// * If the input vector (`utf8_cp`) of UTF-8 code points contains invalid continuation bytes.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81]; // Array of code points in UTF-8
/// let dec: Vec<u32> = decode_from_utf8(&v);
/// assert_eq!(dec, vec![0x10001]);
/// ```
pub fn decode_from_utf8<T: AsRef<Vec<u8>>>(utf8_cp: T) -> Vec<u32> {
    let utf8_cp = utf8_cp.as_ref();
    let len: usize = utf8_cp.len();
    let mut i: usize = 0;
    let mut unicode_cp: Vec<u32> = Vec::new();
    while i < len {
        let (cp, offset) = decode_symbol(utf8_cp, i).unwrap();
        i += offset;
        unicode_cp.push(cp);
    }
    unicode_cp
}
