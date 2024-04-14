/*!
Unicode utilities.

# Introduction

Unicode, formally The Unicode Standard, is a text encoding standard maintained by the Unicode Consortium designed to support the use of text written in all of the world's major writing systems.

Unicode text is processed and stored as binary data using one of several encodings, which define how to translate the standard's abstracted codes for characters into sequences of bytes. The Unicode Standard itself defines three encodings: UTF-8, UTF-16, and UTF-32, though several others exist. Of these, UTF-8 is the most widely used by a large margin, in part due to its backwards-compatibility with ASCII.
*/

use crate::utf8;

/// Pretty print the unicode code points in hexadecimal, (binary) and decimal of a vector of unicode code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the unicode code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in unicode.
fn print_unicode_vec<T: AsRef<Vec<u32>>>(unicode_cp: T, binary_flag: bool) {
    let v: Vec<u32> = unicode_cp.as_ref().to_vec();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    let string_repr: String = String::from_utf8(utf8::encode_in_utf8(&v)).unwrap();
    println!();
    println!(
        "--------------- UNICODE of \"{}\" ---------------",
        string_repr
    );
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!(
        "{}{}",
        "-".repeat(45),
        "-".repeat(string_repr.chars().count())
    );

    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

/// Check if a unicode code point is valid.
/// A unicode code point is valid if it is not in the range `0xD800` to `0xDFFF`.
/// These code points are reserved for UTF-16 surrogate pairs.
///
/// # Parameters
/// * `code_point`: [`u32`] - A unicode code point.
///
/// # Panics
/// * If the input unicode code point is in the range `0xD800` to `0xDFFF`.
pub fn check_code_point(code_point: u32) {
    if (0xD800..=0xDFFF).contains(&code_point) {
        panic!("Invalid UCS-2 sequence {}", code_point);
    }
}

/// Pretty print the unicode code points in hexadecimal and decimal of a vector of unicode code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in unicode.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u32> = vec![0x10001];
/// print_unicode_b(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UNICODE code points ---------------
/// Hex: [10001]
/// Bin: ["10000000000000001"]
/// Dec: [65537]
/// ---------------------------------------------------
/// ```
pub fn print_unicode_b<T: AsRef<Vec<u32>>>(unicode_cp: T) {
    print_unicode_vec(unicode_cp, true);
}

/// Pretty print the unicode code points in hexadecimal and decimal of a vector of unicode code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in unicode.
///
/// # Example
/// ```rust
/// use ende::prelude::*;
/// let v: Vec<u32> = vec![0x10001];
/// print_unicode(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UNICODE code points ---------------
/// Hex: [10001]
/// Dec: [65537]
/// ---------------------------------------------------
/// ```
pub fn print_unicode<T: AsRef<Vec<u32>>>(unicode_cp: T) {
    print_unicode_vec(unicode_cp, false);
}
