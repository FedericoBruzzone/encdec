/*!
UTF-16 encoding and decoding.

# Encoding
A unicode code point is represented using two or four bytes in UTF-16, depending its value.
* If the unicode code point is less than 0xFFFF, it is represented using [a word of 16 bits (two bytes)](#two-bytes-one-word).
* If the unicode code point is greater than or equal to 0xFFFF, it is represented using a surrogate pair [two words of 16 bits (four bytes)](#four-bytes-two-words---surrogate-pair).

# Decoding
A UTF-16 code point is decoded into a unicode code point using the following rules.
* If the UTF-16 code point is less than 0xD800 or greater than 0xDBFF and less than 0xFFFF, it is a unicode code point.
* If the UTF-16 code point is between 0xD800 and 0xDBFF, it is a [surrogate pair](#two-bytes-one-word), so two UTF-16 code points are required to represent a unicode code point.

## Representation

**Note**:

* There are not unicode code points in the range from 0xD800 to 0xDBFF because by unicode specification these values are reserved for surrogate pairs. So, every code point from 0x0 to 0xFFFF is a valid character.
* UTF-16  is capable of encoding all 1,112,064 (2^16 + 2^20 - (0xDFFF - 0xD800 - 1)) valid code points of Unicode.

### Two bytes (one word)

**Encoding**: If the unicode code point is less than 0xFFFF, the unicode code point is represented in UTF-16 using only the 16 least significant bits.

**Decoding**: If the UTF-16 code point is less than 0xD800 or greater than 0xDBFF and less than 0xFFFF, the unicode code point is represented using only the 16 least significant bits.

* Unicode code point: `nnnnnnnn|nnnnnnnn|xxxxxxxx|xxxxxxxx`
* UTF-16 code point: `xxxxxxxx|xxxxxxxx`

### Four bytes (two words) - Surrogate pair

**Encoding**: If the unicode code point is greater than or equal to 0xFFFF, the unicode code point is represented in UTF-16 using a surrogate pair.

**Decoding**: If the UTF-16 code point is between 0xD800 and 0xDBFF it is a surrogate pair, and the unicode code point is represented using the first 10 bits of the first UTF-16 code point and the first ten least significant bits (excluding the prefix bits).

* Unicode code point: `nnnnnnnn|nnnnyyyy|yyyyyyxx|xxxxxxxx`
* UTF-16 code point: `110110yy|yyyyyyyy|110111xx|xxxxxxxx`

**Surrogate pair**

* The first UTF-16 code point is a high surrogate and the second UTF-16 code point is a low surrogate.
* The high surrogate is in the range 0xD800 to 0xDBFF.
* The low surrogate is in the range 0xDC00 to 0xDFFF.
*/

/// Encode a unicode code point into a vector of UTF-16 code points.
///
/// # Parameters
/// * `unicode_cp`: [`u32`] - A unicode code point.
///
/// # Returns
/// A [`Vec<u16>`] containing the UTF-16 code points.
///
/// # Panics
/// * If the input unicode code point is invalid.
fn encode_code_point(unicode_cp: u32) -> Vec<u16> {
    if unicode_cp < 0xFFFF {
        return vec![unicode_cp as u16];
    }

    // unicode_cp: 0x10001 -> 0b0001_0000_0000_0000_0001
    // 0x3FF -> 0b0000_0011_1111_1111
    //                   ^^^^^^^^^^^^__ To be sure that the value is 10 bits
    let mut byte_vec: Vec<u16> = Vec::new();

    // 0x10000 -> 0b0001_0000_0000_0000_0000
    // 0xD800 -> 0b1101_1000_0000_0000 (Default high surrogate)
    //
    // 0b0001_0000_0000_0000_0001 - 0b0001_0000_0000_0000_0000 = 0b0000_0000_0000_0001
    // 0b0000_0000_0000_0001 >> 10 = 0b0000_0000_0000_0000
    // 0b0000_0000_0000_0000 & 0b0000_0011_1111_1111 = 0b0000_0000_0000_0000
    // 0b0000_0000_0000_0000 + 0b1101_1000_0000_0000 = 0b1101_1000_0000_0000
    let extra = unicode_cp - 0x10000;
    let high_surrogate: u16 = (((extra >> 10) & 0x3FF) + 0xD800) as u16;
    byte_vec.push(high_surrogate);

    // 0xDC00 -> 0b1101_1100_0000_0000 (Default low surrogate)
    //
    // 0b0001_0000_0000_0000_0001 & 0b0000_0011_1111_1111 = 0b0000_0000_0000_0000_0001
    // 0b0000_0000_0000_0000_0001 | 0b1101_1100_0000_0000 = 0b1101_1100_0000_0001
    let low_surrogate: u16 = (unicode_cp & 0x3FF | 0xDC00) as u16;
    byte_vec.push(low_surrogate);

    byte_vec
}

/// Decode a UTF-16 code point into a unicode code point.
///
/// # Parameters
/// * `utf16_cp`: [`&[u16]`] - A slice of UTF-16 code points.
/// * `i`: [`usize`] - The index of the byte to read.
///
/// # Returns
/// A tuple containing the unicode code point and the number of bytes read.
/// * The unicode code point is the decoded UTF-16 code point.
/// * The number of bytes read is the number of consumed bytes from the vector of UTF-16 code points.
///
/// # Panics
/// * If the index `i` is out of bounds.
/// * If the UTF-16 code point is invalid.
fn decode_symbol(utf16_cp: &[u16], i: usize) -> Option<(u32, usize)> {
    if i > utf16_cp.len() {
        panic!("Index out of bounds");
    }

    if i == utf16_cp.len() {
        return None;
    }

    let mut code_point: u32;
    let mut offset: usize = 0;

    code_point = utf16_cp[i] as u32;
    offset += 1;
    if !(0xD800..=0xDBFF).contains(&code_point) {
        return Some((code_point, offset));
    }

    let high_surrogate = code_point;
    let low_surrogate = utf16_cp[i + 1] as u32;
    offset += 1;
    if (low_surrogate & 0xFC00) == 0xDC00 {
        // utf16_cp: 0xD800, 0xDC01 -> 0b1101_1000_0000_0000, 0b1101_1100_0000_0001
        // 0xFC00 -> 0b1111_1100_0000_0000
        //             ^^^^^^^______________ In order to compare (keep) the first 6 bits
        // 0xDC00 -> 0b1101_1100_0000_0000
        //             ^^^^^^^______________ The start of the low surrogate
        // 0x3FF -> 0b0000_0011_1111_1111
        //                   ^^^^^^^^^^^^__ To be sure that the value is 10 bits
        // 0x10000 -> 0b0001_0000_0000_0000_0000
        //
        // 0b1101_1000_0000_0000 & 0b0000_0011_1111_1111 = 0b0000_0000_0000_0000
        // 0b0000_0000_0000_0000 << 10 = 0b0000_0000_0000_0000_0000
        //
        // 0b1101_1100_0000_0001 & 0b0000_0011_1111_1111 = 0b0000_0000_0000_0001
        //
        //      0b0000_0000_0000_0000 +
        //      0b0000_0000_0000_0001 +
        // 0b0001_0000_0000_0000_0000 =
        // 0b0001_0000_0000_0000_0001
        code_point = (((high_surrogate & 0x3FF) << 10) + (low_surrogate & 0x3FF)) + 0x10000;
        return Some((code_point, offset));
    }
    panic!("Invalid UCS-2 sequence");
}

/// Pretty print the UTF-16 code points in hexadecimal, (binary) and decimal.
///
/// # Parameters
/// * `utf16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
fn print_utf16_vec<T: AsRef<Vec<u16>>>(utf16_cp: T, binary_flag: bool) {
    let v: Vec<u16> = utf16_cp.as_ref().to_vec();
    let string_repr: String = String::from_utf16(&v).unwrap();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    println!();
    println!(
        "--------------- UTF-16 of \"{}\" ---------------",
        string_repr
    );
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!(
        "{}{}",
        "-".repeat(44),
        "-".repeat(string_repr.chars().count())
    );
    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

/// Pretty print the UTF-16 encoding in hexadecimal and decimal of a vector of UTF-16 code points.
///
/// # Parameters
/// * `uft16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC00];
/// print_utf16(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀁" ---------------
/// Hex: [d800, dc00]
/// Dec: [55296, 56320]
/// ------------------------------------------------------
pub fn print_utf16<T: AsRef<Vec<u16>>>(utf16_cp: T) {
    print_utf16_vec(utf16_cp, false);
}

/// Pretty print the UTF-16 encoding in hexadecimal and decimal of a vector of UTF-16 code points.
///
/// # Parameters
/// * `uft16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC00];
/// print_utf16(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀀" ---------------
/// Hex: [d800, dc00]
/// Bin: ["1101100000000000", "1101110000000000"]
/// Dec: [55296, 56320]
/// ------------------------------------------------------
pub fn print_utf16_b<T: AsRef<Vec<u16>>>(utf16_cp: T) {
    print_utf16_vec(utf16_cp, true);
}

/// Encode a vector of unicode code points into a vector of UTF-16 code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Returns
/// A [`Vec<u16>`] containing the UTF-16 code points.
///
/// # Panics
/// * If the input vector (`unicode_cp`) of unicode code points contains invalid unicode code points.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u32> = vec![0x10001]; // Array of code points in unicode
/// let enc: Vec<u16> = encode_in_utf16(&v);
/// assert_eq!(enc, vec![0xD800, 0xDC01]);
/// ```
pub fn encode_in_utf16<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u16> {
    let unicode_cp: Vec<u32> = unicode_cp.as_ref().to_vec();
    let len: usize = unicode_cp.len();
    let mut utf16_cp: Vec<u16> = Vec::new();
    for cp in unicode_cp.iter().take(len) {
        utf16_cp.append(&mut encode_code_point(*cp));
    }
    utf16_cp
}

/// Decode a vector of UTF-16 code points into a vector of unicode code points.
///
/// # Parameters
/// * `utf16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Returns
/// A [`Vec<u32>`] containing the unicode code points.
///
/// # Panics
/// * If the input vector (`utf16_cp`) of UTF-16 code points contains invalid code points.
/// * If the input vector (`utf16_cp`) of UTF-16 code points contains invalid continuation bytes.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC01]; // Array of code points in UTF-16
/// let dec: Vec<u32> = decode_from_utf16(&v);
/// assert_eq!(dec, vec![0x10001]);
/// ```
pub fn decode_from_utf16<T: AsRef<Vec<u16>>>(utf16_cp: T) -> Vec<u32> {
    let utf16_cp: Vec<u16> = utf16_cp.as_ref().to_vec();
    let len: usize = utf16_cp.len();
    let mut i = 0;
    let mut unicode_cp: Vec<u32> = Vec::new();
    while i < len {
        let (cp, offset) = decode_symbol(&utf16_cp, i).unwrap();
        i += offset;
        unicode_cp.push(cp);
    }
    unicode_cp
}
