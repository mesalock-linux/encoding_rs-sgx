// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_euc_kr(bytes: &[u8], expect: &str) {
    decode(EUC_KR, bytes, expect);
}

fn encode_euc_kr(string: &str, expect: &[u8]) {
    encode(EUC_KR, string, expect);
}

//#[test]
pub fn test_euc_kr_decode() {
    // Empty
    decode_euc_kr(b"", &"");

    // ASCII
    decode_euc_kr(b"\x61\x62", "\u{0061}\u{0062}");

    decode_euc_kr(b"\x81\x41", "\u{AC02}");
    decode_euc_kr(b"\x81\x5B", "\u{FFFD}\x5B");
    decode_euc_kr(b"\xFD\xFE", "\u{8A70}");
    decode_euc_kr(b"\xFE\x41", "\u{FFFD}\x41");
    decode_euc_kr(b"\xFF\x41", "\u{FFFD}\x41");
    decode_euc_kr(b"\x80\x41", "\u{FFFD}\x41");
    decode_euc_kr(b"\xA1\xFF", "\u{FFFD}");
    decode_euc_kr(b"\x81\xFF", "\u{FFFD}");
}

//#[test]
pub fn test_euc_kr_encode() {
    // Empty
    encode_euc_kr("", b"");

    // ASCII
    encode_euc_kr("\u{0061}\u{0062}", b"\x61\x62");

    encode_euc_kr("\u{AC02}", b"\x81\x41");
    encode_euc_kr("\u{8A70}", b"\xFD\xFE");
}

//#[test]
pub fn test_euc_kr_decode_all() {
    let input = include_bytes!("test_data/euc_kr_in.txt");
    let expectation = include_str!("test_data/euc_kr_in_ref.txt");
    let (cow, had_errors) = EUC_KR.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_euc_kr_encode_all() {
    let input = include_str!("test_data/euc_kr_out.txt");
    let expectation = include_bytes!("test_data/euc_kr_out_ref.txt");
    let (cow, encoding, had_errors) = EUC_KR.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, EUC_KR);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_euc_kr_encode_from_two_low_surrogates() {
    let expectation = b"&#65533;&#65533;";
    let mut output = [0u8; 40];
    let mut encoder = EUC_KR.new_encoder();
    let (result, read, written, had_errors) =
        encoder.encode_from_utf16(&[0xDC00u16, 0xDEDEu16], &mut output[..], true);
    assert_eq!(result, CoderResult::InputEmpty);
    assert_eq!(read, 2);
    assert_eq!(written, expectation.len());
    assert!(had_errors);
    assert_eq!(&output[..written], expectation);
}
