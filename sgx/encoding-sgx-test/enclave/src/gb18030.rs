//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_gb18030(bytes: &[u8], expect: &str) {
    decode(GB18030, bytes, expect);
}

fn encode_gb18030(string: &str, expect: &[u8]) {
    encode(GB18030, string, expect);
}

fn encode_gbk(string: &str, expect: &[u8]) {
    encode(GBK, string, expect);
}

//#[test]
pub fn test_gb18030_decode() {
    // Empty
    decode_gb18030(b"", &"");

    // ASCII
    decode_gb18030(b"\x61\x62", "\u{0061}\u{0062}");

    // euro
    decode_gb18030(b"\x80", "\u{20AC}");
    decode_gb18030(b"\xA2\xE3", "\u{20AC}");

    // two bytes
    decode_gb18030(b"\x81\x40", "\u{4E02}");
    decode_gb18030(b"\x81\x7E", "\u{4E8A}");
    decode_gb18030(b"\x81\x7F", "\u{FFFD}\u{007F}");
    decode_gb18030(b"\x81\x80", "\u{4E90}");
    decode_gb18030(b"\x81\xFE", "\u{4FA2}");
    decode_gb18030(b"\xFE\x40", "\u{FA0C}");
    decode_gb18030(b"\xFE\x7E", "\u{E843}");
    decode_gb18030(b"\xFE\x7F", "\u{FFFD}\u{007F}");
    decode_gb18030(b"\xFE\x80", "\u{4723}");
    decode_gb18030(b"\xFE\xFE", "\u{E4C5}");

    // The difference from the original GB18030
    decode_gb18030(b"\xA3\xA0", "\u{3000}");
    decode_gb18030(b"\xA1\xA1", "\u{3000}");

    // 0xFF
    decode_gb18030(b"\xFF\x40", "\u{FFFD}\u{0040}");
    decode_gb18030(b"\xE3\xFF\x9A\x33", "\u{FFFD}\u{FFFD}"); // not \u{FFFD}\u{FFFD}\u{0033} !
    decode_gb18030(b"\xFF\x32\x9A\x33", "\u{FFFD}\u{0032}\u{FFFD}"); // not \u{FFFD}\u{0032}\u{FFFD}\u{0033} !
    decode_gb18030(b"\xFF\x40\x00", "\u{FFFD}\u{0040}\u{0000}");
    decode_gb18030(b"\xE3\xFF\x9A\x33\x00", "\u{FFFD}\u{FFFD}\u{0033}\u{0000}");
    decode_gb18030(
        b"\xFF\x32\x9A\x33\x00",
        "\u{FFFD}\u{0032}\u{FFFD}\u{0033}\u{0000}",
    );

    // Four bytes
    decode_gb18030(b"\x81\x30\x81\x30", "\u{0080}");
    decode_gb18030(b"\x81\x35\xF4\x37", "\u{E7C7}");
    decode_gb18030(b"\x81\x37\xA3\x30", "\u{2603}");
    decode_gb18030(b"\x94\x39\xDA\x33", "\u{1F4A9}");
    decode_gb18030(b"\xE3\x32\x9A\x35", "\u{10FFFF}");
    decode_gb18030(b"\xE3\x32\x9A\x36\x81\x30", "\u{FFFD}\u{FFFD}");
    decode_gb18030(b"\xE3\x32\x9A\x36\x81\x40", "\u{FFFD}\u{4E02}");
    decode_gb18030(b"\xE3\x32\x9A", "\u{FFFD}"); // not \u{FFFD}\u{0032}\u{FFFD} !
    decode_gb18030(b"\xE3\x32\x9A\x00", "\u{FFFD}\u{0032}\u{FFFD}\u{0000}");
}

//#[test]
pub fn test_gb18030_encode() {
    // Empty
    encode_gb18030("", b"");

    // ASCII
    encode_gb18030("\u{0061}\u{0062}", b"\x61\x62");

    // euro
    encode_gb18030("\u{20AC}", b"\xA2\xE3");

    // two bytes
    encode_gb18030("\u{4E02}", b"\x81\x40");
    encode_gb18030("\u{4E8A}", b"\x81\x7E");
    encode_gb18030("\u{4E90}", b"\x81\x80");
    encode_gb18030("\u{4FA2}", b"\x81\xFE");
    encode_gb18030("\u{FA0C}", b"\xFE\x40");
    encode_gb18030("\u{E843}", b"\xFE\x7E");
    encode_gb18030("\u{4723}", b"\xFE\x80");
    encode_gb18030("\u{E4C5}", b"\xFE\xFE");

    // The difference from the original GB18030
    encode_gb18030("\u{E5E5}", b"&#58853;");
    encode_gb18030("\u{3000}", b"\xA1\xA1");

    // Four bytes
    encode_gb18030("\u{0080}", b"\x81\x30\x81\x30");
    encode_gb18030("\u{E7C7}", b"\x81\x35\xF4\x37");
    encode_gb18030("\u{2603}", b"\x81\x37\xA3\x30");
    encode_gb18030("\u{1F4A9}", b"\x94\x39\xDA\x33");
    encode_gb18030("\u{10FFFF}", b"\xE3\x32\x9A\x35");

    // Edge cases
    encode_gb18030("\u{00F7}", b"\xA1\xC2");
}

//#[test]
pub fn test_gbk_encode() {
    // Empty
    encode_gbk("", b"");

    // ASCII
    encode_gbk("\u{0061}\u{0062}", b"\x61\x62");

    // euro
    encode_gbk("\u{20AC}", b"\x80");

    // two bytes
    encode_gbk("\u{4E02}", b"\x81\x40");
    encode_gbk("\u{4E8A}", b"\x81\x7E");
    encode_gbk("\u{4E90}", b"\x81\x80");
    encode_gbk("\u{4FA2}", b"\x81\xFE");
    encode_gbk("\u{FA0C}", b"\xFE\x40");
    encode_gbk("\u{E843}", b"\xFE\x7E");
    encode_gbk("\u{4723}", b"\xFE\x80");
    encode_gbk("\u{E4C5}", b"\xFE\xFE");

    // The difference from the original gb18030
    encode_gbk("\u{E5E5}", b"&#58853;");
    encode_gbk("\u{3000}", b"\xA1\xA1");

    // Four bytes
    encode_gbk("\u{0080}", b"&#128;");
    encode_gbk("\u{E7C7}", b"&#59335;");
    encode_gbk("\u{2603}", b"&#9731;");
    encode_gbk("\u{1F4A9}", b"&#128169;");
    encode_gbk("\u{10FFFF}", b"&#1114111;");

    // Edge cases
    encode_gbk("\u{00F7}", b"\xA1\xC2");
}

//#[test]
pub fn test_gb18030_decode_all() {
    let input = include_bytes!("test_data/gb18030_in.txt");
    let expectation = include_str!("test_data/gb18030_in_ref.txt");
    let (cow, had_errors) = GB18030.decode_without_bom_handling(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_gb18030_encode_all() {
    let input = include_str!("test_data/gb18030_out.txt");
    let expectation = include_bytes!("test_data/gb18030_out_ref.txt");
    let (cow, encoding, had_errors) = GB18030.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, GB18030);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_gb18030_encode_from_utf16_max_length() {
    let mut output = [0u8; 20];
    let mut encoder = GB18030.new_encoder();
    {
        let needed = encoder
            .max_buffer_length_from_utf16_without_replacement(1)
            .unwrap();
        let (result, read, written) = encoder.encode_from_utf16_without_replacement(
            &[0x3000],
            &mut output[..needed],
            true,
        );
        assert_eq!(result, EncoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 2);
        assert_eq!(output[0], 0xA1);
        assert_eq!(output[1], 0xA1);
    }
}
