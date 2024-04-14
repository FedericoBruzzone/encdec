use ende::prelude::*;

#[test]
fn test_utf16_encode_in_utf16_1() {
    let v: Vec<u32> = vec![0x10001];
    let enc: Vec<u16> = encode_in_utf16(v);
    assert_eq!(enc, vec![0xd800, 0xdc01]);
}

#[test]
fn test_utf16_decode_from_utf16_1() {
    let enc: Vec<u16> = vec![0xd800, 0xdc01];
    let v: Vec<u32> = decode_from_utf16(enc);
    assert_eq!(v, vec![0x10001]);
}

#[test]
fn test_utf16_encode_in_utf16_2() {
    let v: Vec<u32> = vec![0x10001, 0x10001];
    let enc: Vec<u16> = encode_in_utf16(v);
    assert_eq!(enc, vec![0xd800, 0xdc01, 0xd800, 0xdc01]);
}

#[test]
fn test_utf16_decode_from_utf16_2() {
    let enc: Vec<u16> = vec![0xd800, 0xdc01, 0xd800, 0xdc01];
    let v: Vec<u32> = decode_from_utf16(enc);
    assert_eq!(v, vec![0x10001, 0x10001]);
}

#[test]
fn test_utf16_encode_in_utf16_3() {
    let v: Vec<u32> = vec![0x10001, 0x10001, 0x10001];
    let enc: Vec<u16> = encode_in_utf16(v);
    assert_eq!(enc, vec![0xd800, 0xdc01, 0xd800, 0xdc01, 0xd800, 0xdc01]);
}

#[test]
fn test_utf16_decode_from_utf16_3() {
    let enc: Vec<u16> = vec![0xd800, 0xdc01, 0xd800, 0xdc01, 0xd800, 0xdc01];
    let v: Vec<u32> = decode_from_utf16(enc);
    assert_eq!(v, vec![0x10001, 0x10001, 0x10001]);
}

#[test]
fn test_utf16_encode_in_utf16_4() {
    let v: Vec<u32> = vec![0x10001, 0x23456];
    let enc: Vec<u16> = encode_in_utf16(v);
    assert_eq!(enc, vec![0xd800, 0xdc01, 0xd84d, 0xdc56]);
}

#[test]
fn test_utf16_decode_from_utf16_4() {
    let enc: Vec<u16> = vec![0xd800, 0xdc01, 0xd84d, 0xdc56];
    let v: Vec<u32> = decode_from_utf16(enc);
    assert_eq!(v, vec![0x10001, 0x23456]);
}
