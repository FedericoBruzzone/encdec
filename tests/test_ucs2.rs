use ende::prelude::*;

#[test]
fn test_ucs2_encode_in_ucs2_1() {
    let v: Vec<u32> = vec![0xffee];
    let enc: Vec<u16> = encode_in_ucs2(v);
    assert_eq!(enc, vec![0xffee]);
}

#[test]
fn test_ucs2_decode_from_ucs2_1() {
    let enc: Vec<u16> = vec![0xffee];
    let v: Vec<u32> = decode_from_ucs2(enc);
    assert_eq!(v, vec![0xffee]);
}

#[test]
fn test_ucs2_encode_in_ucs2_2() {
    let v: Vec<u32> = vec![0xffee, 0xffee];
    let enc: Vec<u16> = encode_in_ucs2(v);
    assert_eq!(enc, vec![0xffee, 0xffee]);
}

#[test]
fn test_ucs2_decode_from_ucs2_2() {
    let enc: Vec<u16> = vec![0xffee, 0xffee];
    let v: Vec<u32> = decode_from_ucs2(enc);
    assert_eq!(v, vec![0xffee, 0xffee]);
}
