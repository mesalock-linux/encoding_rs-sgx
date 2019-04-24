//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_replacement(bytes: &[u8], expect: &str) {
    decode_without_padding(REPLACEMENT, bytes, expect);
}

fn encode_replacement(string: &str, expect: &[u8]) {
    encode(REPLACEMENT, string, expect);
}

//#[test]
pub fn test_replacement_decode() {
    decode_replacement(b"", "");
    decode_replacement(b"A", "\u{FFFD}");
    decode_replacement(b"AB", "\u{FFFD}");
}

//#[test]
pub fn test_replacement_encode() {
    // Empty
    encode_replacement("", b"");

    assert_eq!(REPLACEMENT.new_encoder().encoding(), UTF_8);
    encode_replacement("\u{1F4A9}\u{2603}", "\u{1F4A9}\u{2603}".as_bytes());
}
