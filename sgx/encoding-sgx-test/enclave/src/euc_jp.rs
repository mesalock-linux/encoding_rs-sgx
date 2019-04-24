//use super::super::testing::*;
//use super::super::*;

use encoding_rs::*;
use crate::testing::*;

fn decode_euc_jp(bytes: &[u8], expect: &str) {
    decode(EUC_JP, bytes, expect);
}

fn encode_euc_jp(string: &str, expect: &[u8]) {
    encode(EUC_JP, string, expect);
}

//#[test]
pub fn test_euc_jp_decode() {
    // Empty
    decode_euc_jp(b"", &"");

    // ASCII
    decode_euc_jp(b"\x61\x62", "\u{0061}\u{0062}");

    // Half-width
    decode_euc_jp(b"\x8E\xA1", "\u{FF61}");
    decode_euc_jp(b"\x8E\xDF", "\u{FF9F}");
    decode_euc_jp(b"\x8E\xA0", "\u{FFFD}");
    decode_euc_jp(b"\x8E\xE0", "\u{FFFD}");
    decode_euc_jp(b"\x8E\xFF", "\u{FFFD}");
    decode_euc_jp(b"\x8E", "\u{FFFD}");

    // JIS 0212
    decode_euc_jp(b"\x8F\xA1\xA1", "\u{FFFD}");
    decode_euc_jp(b"\x8F\xA2\xAF", "\u{02D8}");
    decode_euc_jp(b"\x8F\xA2\xFF", "\u{FFFD}");
    decode_euc_jp(b"\x8F\xA1", "\u{FFFD}");
    decode_euc_jp(b"\x8F", "\u{FFFD}");

    // JIS 0208
    decode_euc_jp(b"\xA1\xA1", "\u{3000}");
    decode_euc_jp(b"\xA1\xA0", "\u{FFFD}");
    decode_euc_jp(b"\xFC\xFE", "\u{FF02}");
    decode_euc_jp(b"\xFE\xFE", "\u{FFFD}");
    decode_euc_jp(b"\xA1", "\u{FFFD}");

    // Bad leads
    decode_euc_jp(b"\xFF\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\xA0\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x80\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x81\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x82\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x83\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x84\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x85\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x86\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x87\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x88\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x89\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x8A\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x8B\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x8C\xA1\xA1", "\u{FFFD}\u{3000}");
    decode_euc_jp(b"\x8D\xA1\xA1", "\u{FFFD}\u{3000}");

    // Bad ASCII trail
    decode_euc_jp(b"\xA1\x40", "\u{FFFD}\u{0040}");
}

//#[test]
pub fn test_euc_jp_encode() {
    // Empty
    encode_euc_jp("", b"");

    // ASCII
    encode_euc_jp("\u{0061}\u{0062}", b"\x61\x62");

    // Exceptional code points
    encode_euc_jp("\u{00A5}", b"\x5C");
    encode_euc_jp("\u{203E}", b"\x7E");
    encode_euc_jp("\u{2212}", b"\xA1\xDD");

    // Half-width
    encode_euc_jp("\u{FF61}", b"\x8E\xA1");
    encode_euc_jp("\u{FF9F}", b"\x8E\xDF");

    // JIS 0212
    encode_euc_jp("\u{02D8}", b"&#728;");

    // JIS 0208
    encode_euc_jp("\u{3000}", b"\xA1\xA1");
    encode_euc_jp("\u{FF02}", b"\xFC\xFE");
}

//#[test]
pub fn test_jis0208_decode_all() {
    let input = include_bytes!("test_data/jis0208_in.txt");
    let expectation = include_str!("test_data/jis0208_in_ref.txt");
    let (cow, had_errors) = EUC_JP.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_jis0208_encode_all() {
    let input = include_str!("test_data/jis0208_out.txt");
    let expectation = include_bytes!("test_data/jis0208_out_ref.txt");
    let (cow, encoding, had_errors) = EUC_JP.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, EUC_JP);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_jis0212_decode_all() {
    let input = include_bytes!("test_data/jis0212_in.txt");
    let expectation = include_str!("test_data/jis0212_in_ref.txt");
    let (cow, had_errors) = EUC_JP.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}
