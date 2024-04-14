use ende::prelude::*;

#[test]
fn test_utf8_encode_in_utf8_1() {
    let v: Vec<u32> = vec![0x10001];
    let enc: Vec<u8> = encode_in_utf8(v);
    assert_eq!(enc, vec![0xf0, 0x90, 0x80, 0x81]);
}

#[test]
fn test_utf8_decode_from_utf8_1() {
    let enc: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81];
    let v: Vec<u32> = decode_from_utf8(enc);
    assert_eq!(v, vec![0x10001]);
}

#[test]
fn test_utf8_encode_in_utf8_2() {
    let v: Vec<u32> = vec![0x10001, 0x10001];
    let enc: Vec<u8> = encode_in_utf8(v);
    assert_eq!(enc, vec![0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81]);
}

#[test]
fn test_utf8_decode_from_utf8_2() {
    let enc: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81];
    let v: Vec<u32> = decode_from_utf8(enc);
    assert_eq!(v, vec![0x10001, 0x10001]);
}

#[test]
fn test_utf8_encode_in_utf8_3() {
    let v: Vec<u32> = vec![0x10001, 0x10001, 0x10001];
    let enc: Vec<u8> = encode_in_utf8(v);
    assert_eq!(
        enc,
        vec![0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81]
    );
}

#[test]
fn test_utf8_decode_from_utf8_3() {
    let enc: Vec<u8> = vec![
        0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81, 0xf0, 0x90, 0x80, 0x81,
    ];
    let v: Vec<u32> = decode_from_utf8(enc);
    assert_eq!(v, vec![0x10001, 0x10001, 0x10001]);
}

#[test]
fn test_utf8_encode_in_utf8_4() {
    let v: Vec<u32> = vec![0x10001, 0x23456];
    let enc: Vec<u8> = encode_in_utf8(v);
    assert_eq!(enc, vec![0xf0, 0x90, 0x80, 0x81, 0xf0, 0xa3, 0x91, 0x96]);
}

#[test]
fn test_utf8_decode_from_utf8_4() {
    let enc: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81, 0xf0, 0xa3, 0x91, 0x96];
    let v: Vec<u32> = decode_from_utf8(enc);
    assert_eq!(v, vec![0x10001, 0x23456]);
}
