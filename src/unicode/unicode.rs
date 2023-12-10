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
    println!();
    println!("--------------- UNICODE code points ---------------");
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!("{}", "-".repeat(51));
    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

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
/// let v: Vec<u32> = vec![0x10001];
/// utf8::print_unicode_b(&v);
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
/// let v: Vec<u32> = vec![0x10001];
/// utf8::print_unicode(&v);
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