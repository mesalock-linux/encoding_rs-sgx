//#[cfg(test)]
//mod tests {
//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_big5(bytes: &[u8], expect: &str) {
    decode(BIG5, bytes, expect);
}

fn encode_big5(string: &str, expect: &[u8]) {
    encode(BIG5, string, expect);
}

//#[test]
pub fn test_big5_decode() {
    // Empty
    decode_big5(b"", &"");

    // ASCII
    decode_big5(&[0x61u8, 0x62u8], &"\u{0061}\u{0062}");

    // Edge cases
    decode_big5(&[0x87u8, 0x40u8], &"\u{43F0}");
    decode_big5(&[0xFEu8, 0xFEu8], &"\u{79D4}");
    decode_big5(&[0xFEu8, 0xFDu8], &"\u{2910D}");
    decode_big5(&[0x88u8, 0x62u8], &"\u{00CA}\u{0304}");
    decode_big5(&[0x88u8, 0x64u8], &"\u{00CA}\u{030C}");
    decode_big5(&[0x88u8, 0x66u8], &"\u{00CA}");
    decode_big5(&[0x88u8, 0xA3u8], &"\u{00EA}\u{0304}");
    decode_big5(&[0x88u8, 0xA5u8], &"\u{00EA}\u{030C}");
    decode_big5(&[0x88u8, 0xA7u8], &"\u{00EA}");
    decode_big5(&[0x99u8, 0xD4u8], &"\u{8991}");
    decode_big5(&[0x99u8, 0xD5u8], &"\u{27967}");
    decode_big5(&[0x99u8, 0xD6u8], &"\u{8A29}");

    // Edge cases surrounded with ASCII
    decode_big5(
        &[0x61u8, 0x87u8, 0x40u8, 0x62u8],
        &"\u{0061}\u{43F0}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0xFEu8, 0xFEu8, 0x62u8],
        &"\u{0061}\u{79D4}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0xFEu8, 0xFDu8, 0x62u8],
        &"\u{0061}\u{2910D}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0x62u8, 0x62u8],
        &"\u{0061}\u{00CA}\u{0304}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0x64u8, 0x62u8],
        &"\u{0061}\u{00CA}\u{030C}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0x66u8, 0x62u8],
        &"\u{0061}\u{00CA}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0xA3u8, 0x62u8],
        &"\u{0061}\u{00EA}\u{0304}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0xA5u8, 0x62u8],
        &"\u{0061}\u{00EA}\u{030C}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x88u8, 0xA7u8, 0x62u8],
        &"\u{0061}\u{00EA}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x99u8, 0xD4u8, 0x62u8],
        &"\u{0061}\u{8991}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x99u8, 0xD5u8, 0x62u8],
        &"\u{0061}\u{27967}\u{0062}",
    );
    decode_big5(
        &[0x61u8, 0x99u8, 0xD6u8, 0x62u8],
        &"\u{0061}\u{8A29}\u{0062}",
    );

    // Bad sequences
    decode_big5(&[0x80u8, 0x61u8], &"\u{FFFD}\u{0061}");
    decode_big5(&[0xFFu8, 0x61u8], &"\u{FFFD}\u{0061}");
    decode_big5(&[0xFEu8, 0x39u8], &"\u{FFFD}\u{0039}");
    decode_big5(&[0x87u8, 0x66u8], &"\u{FFFD}\u{0066}");
    decode_big5(&[0x81u8, 0x40u8], &"\u{FFFD}\u{0040}");
    decode_big5(&[0x61u8, 0x81u8], &"\u{0061}\u{FFFD}");
}

//#[test]
pub fn test_big5_encode() {
    // Empty
    encode_big5("", b"");

    // ASCII
    encode_big5("\u{0061}\u{0062}", b"\x61\x62");

    // Edge cases
    encode_big5("\u{9EA6}\u{0061}", b"&#40614;\x61");
    encode_big5("\u{2626B}\u{0061}", b"&#156267;\x61");
    encode_big5("\u{3000}", b"\xA1\x40");
    encode_big5("\u{20AC}", b"\xA3\xE1");
    encode_big5("\u{4E00}", b"\xA4\x40");
    encode_big5("\u{27607}", b"\xC8\xA4");
    encode_big5("\u{FFE2}", b"\xC8\xCD");
    encode_big5("\u{79D4}", b"\xFE\xFE");

    // Not in index
    encode_big5("\u{2603}\u{0061}", b"&#9731;\x61");

    // duplicate low bits
    encode_big5("\u{203B5}", b"\xFD\x6A");
    encode_big5("\u{25605}", b"\xFE\x46");

    // prefer last
    encode_big5("\u{2550}", b"\xF9\xF9");
}

//#[test]
pub fn test_big5_decode_all() {
    let input = include_bytes!("test_data/big5_in.txt");
    let expectation = include_str!("test_data/big5_in_ref.txt");
    let (cow, had_errors) = BIG5.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_big5_encode_all() {
    let input = include_str!("test_data/big5_out.txt");
    let expectation = include_bytes!("test_data/big5_out_ref.txt");
    let (cow, encoding, had_errors) = BIG5.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, BIG5);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_big5_encode_from_two_low_surrogates() {
    let expectation = b"&#65533;&#65533;";
    let mut output = [0u8; 40];
    let mut encoder = BIG5.new_encoder();
    let (result, read, written, had_errors) =
        encoder.encode_from_utf16(&[0xDC00u16, 0xDEDEu16], &mut output[..], true);
    assert_eq!(result, CoderResult::InputEmpty);
    assert_eq!(read, 2);
    assert_eq!(written, expectation.len());
    assert!(had_errors);
    assert_eq!(&output[..written], expectation);
}
//}
