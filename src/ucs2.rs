/*!
UCS-2 encoding and decoding.

# Encoding
A unicode code point is represented using [two bytes]() in UCS-2, using always this fixed size.

# Decoding
A UCS-2 code point is decoded into a unicode code point using the the first [two bytes]().

## Representation

**Note**:

* UCS-2 is a subset of UTF-16.
* UCS-2 is capable of ending 65,536 code points. This is the same as the first 65,536 code points of UTF-16.

### Two bytes

**Encoding**: If the unicode code point is less than 0xFFFF, the unicode code point is represented in UTF-16 using only the 16 least significant bits.

**Decoding**: If the UTF-16 code point is less than 0xD800 or greater than 0xDBFF and less than 0xFFFF, the unicode code point is represented using only the 16 least significant bits.

* Unicode code point: `nnnnnnnn|nnnnnnnn|xxxxxxxx|xxxxxxxx`
* UTF-16 code point: `xxxxxxxx|xxxxxxxx`
*/

/// Pretty print the UCS-2 code points in hexadecimal, (binary) and decimal.
///
/// # Parameters
/// * `ucs2_cp`: [`Vec<u16>`] - A vector of UCS-2 code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the UCS-2 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UCS-2.
fn print_ucs2_vec<T: AsRef<Vec<u16>>>(ucs2_cp: T, binary_flag: bool) {
    let v: Vec<u16> = ucs2_cp.as_ref().to_vec();
    let string_repr: String = String::from_utf16(&v).unwrap();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    println!();
    println!(
        "--------------- UCS-2 of \"{}\" ---------------",
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
/// Pretty print the UCS-2 encoding in hexadecimal and decimal of a vector of UCS-2 code points.
///
/// # Parameters
/// * `ucs2_cp`: [`Vec<u16>`] - A vector of UCS-2 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UCS-2.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xFFEE];
/// print_ucs2(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀁" ---------------
/// Hex: [0xFFEE]
/// Dec: [65518]
/// ------------------------------------------------------
pub fn print_ucs2<T: AsRef<Vec<u16>>>(utf2_cp: T) {
    print_ucs2_vec(utf2_cp, false);
}

/// Pretty print the UCS-2 encoding in hexadecimal and decimal of a vector of UCS-2 code points.
///
/// # Parameters
/// * `ucs2_cp`: [`Vec<u16>`] - A vector of UCS-2 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UCS-2.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xFFEE];
/// print_ucs2_b(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀀" ---------------
/// Hex: [0xFFEE]
/// Bin: ["1111111111101110"]
/// Dec: [65518]
/// ------------------------------------------------------
pub fn print_ucs2_b<T: AsRef<Vec<u16>>>(ucs2_cp: T) {
    print_ucs2_vec(ucs2_cp, true);
}
/// Encode a vector of unicode code points into a vector of UCS-2 code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Returns
/// A [`Vec<u16>`] containing the UCS-2 code points.
///
/// # Panics
/// * If the input vector (`unicode_cp`) of unicode code points contains invalid unicode code points.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u32> = vec![0xFFEE]; // Array of code points in unicode
/// let enc: Vec<u16> = encode_in_ucs2(&v);
/// assert_eq!(enc, vec![0xFFEE]);
/// ```
pub fn encode_in_ucs2<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u16> {
    let mut new_v: Vec<u16> = Vec::new();
    let v: Vec<u32> = unicode_cp.as_ref().to_vec();
    for i in &v {
        let code_point = *i;
        if code_point > 0xFFFF {
            panic!("Invalid UCS-2 sequence");
        }
        new_v.push(code_point as u16);
    }
    new_v
}

/// Decode a vector of UCS-2 code points into a vector of unicode code points.
///
/// # Parameters
/// * `ucs2_cp`: [`Vec<u16>`] - A vector of UCS-2 code points.
///
/// # Returns
/// A [`Vec<u32>`] containing the unicode code points.
///
/// # Panics
/// * If the input vector (`ucs2_cp`) of UCS-2 code points contains invalid UCS-2 code points.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u16> = vec![0xFFEE]; // Array of code points in UCS-2
/// let dec: Vec<u32> = decode_from_ucs2(&v);
/// assert_eq!(dec, vec![0xFFEE]);
/// ```
pub fn decode_from_ucs2<T: AsRef<Vec<u16>>>(ucs2_cp: T) -> Vec<u32> {
    let mut new_v: Vec<u32> = Vec::new();
    let v: Vec<u16> = ucs2_cp.as_ref().to_vec();
    let mut i = 0;
    while i < v.len() {
        let code_point = v[i];
        if (0xD800..=0xDBFF).contains(&code_point) {
            panic!("Invalid UCS-2 sequence");
        }
        new_v.push(code_point as u32);
        i += 1;
    }
    new_v
}
