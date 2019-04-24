//use super::super::testing::*;
//use super::super::*;

use encoding_rs::*;
use crate::testing::*;

fn decode_x_user_defined(bytes: &[u8], expect: &str) {
    decode(X_USER_DEFINED, bytes, expect);
}

fn encode_x_user_defined(string: &str, expect: &[u8]) {
    encode(X_USER_DEFINED, string, expect);
}

//#[test]
pub fn test_x_user_defined_decode() {
    // Empty
    decode_x_user_defined(b"", "");

    // ASCII
    decode_x_user_defined(b"\x61\x62", "\u{0061}\u{0062}");

    decode_x_user_defined(b"\x80\xFF", "\u{F780}\u{F7FF}");
    decode_x_user_defined(b"\x80\xFF\x61\x62\x80\xFF\x61\x62\x80\xFF\x61\x62\x80\xFF\x61\x62\x80\xFF\x61\x62", "\u{F780}\u{F7FF}\u{0061}\u{0062}\u{F780}\u{F7FF}\u{0061}\u{0062}\u{F780}\u{F7FF}\u{0061}\u{0062}\u{F780}\u{F7FF}\u{0061}\u{0062}\u{F780}\u{F7FF}\u{0061}\u{0062}");
}

//#[test]
pub fn test_x_user_defined_encode() {
    // Empty
    encode_x_user_defined("", b"");

    // ASCII
    encode_x_user_defined("\u{0061}\u{0062}", b"\x61\x62");

    encode_x_user_defined("\u{F780}\u{F7FF}", b"\x80\xFF");
    encode_x_user_defined("\u{F77F}\u{F800}", b"&#63359;&#63488;");
}

//#[test]
pub fn test_x_user_defined_from_two_low_surrogates() {
    let expectation = b"&#65533;&#65533;";
    let mut output = [0u8; 40];
    let mut encoder = X_USER_DEFINED.new_encoder();
    let (result, read, written, had_errors) =
        encoder.encode_from_utf16(&[0xDC00u16, 0xDEDEu16], &mut output[..], true);
    assert_eq!(result, CoderResult::InputEmpty);
    assert_eq!(read, 2);
    assert_eq!(written, expectation.len());
    assert!(had_errors);
    assert_eq!(&output[..written], expectation);
}
