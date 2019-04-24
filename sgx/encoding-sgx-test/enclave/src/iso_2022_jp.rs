// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_iso_2022_jp(bytes: &[u8], expect: &str) {
    decode(ISO_2022_JP, bytes, expect);
}

fn encode_iso_2022_jp(string: &str, expect: &[u8]) {
    encode(ISO_2022_JP, string, expect);
}

//#[test]
pub fn test_iso_2022_jp_decode() {
    // Empty
    decode_iso_2022_jp(b"", &"");

    // ASCII
    decode_iso_2022_jp(b"\x61\x62", "\u{0061}\u{0062}");
    decode_iso_2022_jp(b"\x7F\x0E\x0F", "\u{007F}\u{FFFD}\u{FFFD}");

    // Partial escapes
    decode_iso_2022_jp(b"\x1B", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$", "\u{FFFD}$");
    decode_iso_2022_jp(b"\x1B(", "\u{FFFD}(");
    decode_iso_2022_jp(b"\x1B.", "\u{FFFD}.");

    // ISO escapes
    decode_iso_2022_jp(b"\x1B(B", ""); // ASCII
    decode_iso_2022_jp(b"\x1B(J", ""); // Roman
    decode_iso_2022_jp(b"\x1B$@", ""); // 0208
    decode_iso_2022_jp(b"\x1B$B", ""); // 0208
    decode_iso_2022_jp(b"\x1B$(D", "\u{FFFD}$(D"); // 2012
    decode_iso_2022_jp(b"\x1B$A", "\u{FFFD}$A"); // GB2312
    decode_iso_2022_jp(b"\x1B$(C", "\u{FFFD}$(C"); // KR
    decode_iso_2022_jp(b"\x1B.A", "\u{FFFD}.A"); // Latin-1
    decode_iso_2022_jp(b"\x1B.F", "\u{FFFD}.F"); // Greek
    decode_iso_2022_jp(b"\x1B(I", ""); // Half-width Katakana
    decode_iso_2022_jp(b"\x1B$(O", "\u{FFFD}$(O"); // 2013
    decode_iso_2022_jp(b"\x1B$(P", "\u{FFFD}$(P"); // 2013
    decode_iso_2022_jp(b"\x1B$(Q", "\u{FFFD}$(Q"); // 2013
    decode_iso_2022_jp(b"\x1B$)C", "\u{FFFD}$)C"); // KR
    decode_iso_2022_jp(b"\x1B$)A", "\u{FFFD}$)A"); // GB2312
    decode_iso_2022_jp(b"\x1B$)G", "\u{FFFD}$)G"); // CNS
    decode_iso_2022_jp(b"\x1B$*H", "\u{FFFD}$*H"); // CNS
    decode_iso_2022_jp(b"\x1B$)E", "\u{FFFD}$)E"); // IR
    decode_iso_2022_jp(b"\x1B$+I", "\u{FFFD}$+I"); // CNS
    decode_iso_2022_jp(b"\x1B$+J", "\u{FFFD}$+J"); // CNS
    decode_iso_2022_jp(b"\x1B$+K", "\u{FFFD}$+K"); // CNS
    decode_iso_2022_jp(b"\x1B$+L", "\u{FFFD}$+L"); // CNS
    decode_iso_2022_jp(b"\x1B$+M", "\u{FFFD}$+M"); // CNS
    decode_iso_2022_jp(b"\x1B$(@", "\u{FFFD}$(@"); // 0208
    decode_iso_2022_jp(b"\x1B$(A", "\u{FFFD}$(A"); // GB2312
    decode_iso_2022_jp(b"\x1B$(B", "\u{FFFD}$(B"); // 0208
    decode_iso_2022_jp(b"\x1B%G", "\u{FFFD}%G"); // UTF-8

    // ASCII
    decode_iso_2022_jp(b"\x5B", "\u{005B}");
    decode_iso_2022_jp(b"\x5C", "\u{005C}");
    decode_iso_2022_jp(b"\x7E", "\u{007E}");
    decode_iso_2022_jp(b"\x0E", "\u{FFFD}");
    decode_iso_2022_jp(b"\x0F", "\u{FFFD}");
    decode_iso_2022_jp(b"\x80", "\u{FFFD}");
    decode_iso_2022_jp(b"\xFF", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x5B", "\u{005B}");
    decode_iso_2022_jp(b"\x1B(B\x5C", "\u{005C}");
    decode_iso_2022_jp(b"\x1B(B\x7E", "\u{007E}");
    decode_iso_2022_jp(b"\x1B(B\x0E", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x0F", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x80", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\xFF", "\u{FFFD}");

    // Roman
    decode_iso_2022_jp(b"\x1B(J\x5B", "\u{005B}");
    decode_iso_2022_jp(b"\x1B(J\x5C", "\u{00A5}");
    decode_iso_2022_jp(b"\x1B(J\x7E", "\u{203E}");
    decode_iso_2022_jp(b"\x1B(J\x0E", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\x0F", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\x80", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\xFF", "\u{FFFD}");

    // Katakana
    decode_iso_2022_jp(b"\x1B(I\x20", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x21", "\u{FF61}");
    decode_iso_2022_jp(b"\x1B(I\x5F", "\u{FF9F}");
    decode_iso_2022_jp(b"\x1B(I\x60", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x0E", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x0F", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x80", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\xFF", "\u{FFFD}");

    // 0208 differences from 1978 to 1983
    decode_iso_2022_jp(b"\x1B$@\x54\x64", "\u{58FA}");
    decode_iso_2022_jp(b"\x1B$@\x44\x5B", "\u{58F7}");
    decode_iso_2022_jp(b"\x1B$@\x74\x21", "\u{582F}");
    decode_iso_2022_jp(b"\x1B$@\x36\x46", "\u{5C2D}");
    decode_iso_2022_jp(b"\x1B$@\x28\x2E", "\u{250F}");
    decode_iso_2022_jp(b"\x1B$B\x54\x64", "\u{58FA}");
    decode_iso_2022_jp(b"\x1B$B\x44\x5B", "\u{58F7}");
    decode_iso_2022_jp(b"\x1B$B\x74\x21", "\u{582F}");
    decode_iso_2022_jp(b"\x1B$B\x36\x46", "\u{5C2D}");
    decode_iso_2022_jp(b"\x1B$B\x28\x2E", "\u{250F}");

    // Broken 0208
    decode_iso_2022_jp(b"\x1B$B\x28\x41", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$@\x80\x54\x64", "\u{FFFD}\u{58FA}");
    decode_iso_2022_jp(b"\x1B$B\x28\x80", "\u{FFFD}");

    // Transitions
    decode_iso_2022_jp(b"\x1B(B\x5C\x1B(J\x5C", "\u{005C}\u{00A5}");
    decode_iso_2022_jp(b"\x1B(B\x5C\x1B(I\x21", "\u{005C}\u{FF61}");
    decode_iso_2022_jp(b"\x1B(B\x5C\x1B$@\x54\x64", "\u{005C}\u{58FA}");
    decode_iso_2022_jp(b"\x1B(B\x5C\x1B$B\x54\x64", "\u{005C}\u{58FA}");

    decode_iso_2022_jp(b"\x1B(J\x5C\x1B(B\x5C", "\u{00A5}\u{005C}");
    decode_iso_2022_jp(b"\x1B(J\x5C\x1B(I\x21", "\u{00A5}\u{FF61}");
    decode_iso_2022_jp(b"\x1B(J\x5C\x1B$@\x54\x64", "\u{00A5}\u{58FA}");
    decode_iso_2022_jp(b"\x1B(J\x5C\x1B$B\x54\x64", "\u{00A5}\u{58FA}");

    decode_iso_2022_jp(b"\x1B(I\x21\x1B(J\x5C", "\u{FF61}\u{00A5}");
    decode_iso_2022_jp(b"\x1B(I\x21\x1B(B\x5C", "\u{FF61}\u{005C}");
    decode_iso_2022_jp(b"\x1B(I\x21\x1B$@\x54\x64", "\u{FF61}\u{58FA}");
    decode_iso_2022_jp(b"\x1B(I\x21\x1B$B\x54\x64", "\u{FF61}\u{58FA}");

    decode_iso_2022_jp(b"\x1B$@\x54\x64\x1B(J\x5C", "\u{58FA}\u{00A5}");
    decode_iso_2022_jp(b"\x1B$@\x54\x64\x1B(I\x21", "\u{58FA}\u{FF61}");
    decode_iso_2022_jp(b"\x1B$@\x54\x64\x1B(B\x5C", "\u{58FA}\u{005C}");
    decode_iso_2022_jp(b"\x1B$@\x54\x64\x1B$B\x54\x64", "\u{58FA}\u{58FA}");

    decode_iso_2022_jp(b"\x1B$B\x54\x64\x1B(J\x5C", "\u{58FA}\u{00A5}");
    decode_iso_2022_jp(b"\x1B$B\x54\x64\x1B(I\x21", "\u{58FA}\u{FF61}");
    decode_iso_2022_jp(b"\x1B$B\x54\x64\x1B$@\x54\x64", "\u{58FA}\u{58FA}");
    decode_iso_2022_jp(b"\x1B$B\x54\x64\x1B(B\x5C", "\u{58FA}\u{005C}");

    // Empty transitions
    decode_iso_2022_jp(b"\x1B(B\x1B(J", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x1B(I", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x1B$@", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(B\x1B$B", "\u{FFFD}");

    decode_iso_2022_jp(b"\x1B(J\x1B(B", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\x1B(I", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\x1B$@", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(J\x1B$B", "\u{FFFD}");

    decode_iso_2022_jp(b"\x1B(I\x1B(J", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x1B(B", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x1B$@", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B(I\x1B$B", "\u{FFFD}");

    decode_iso_2022_jp(b"\x1B$@\x1B(J", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$@\x1B(I", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$@\x1B(B", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$@\x1B$B", "\u{FFFD}");

    decode_iso_2022_jp(b"\x1B$B\x1B(J", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$B\x1B(I", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$B\x1B$@", "\u{FFFD}");
    decode_iso_2022_jp(b"\x1B$B\x1B(B", "\u{FFFD}");

    // Transitions to self
    decode_iso_2022_jp(b"\x1B(B\x5C\x1B(B\x5C", "\u{005C}\u{005C}");
    decode_iso_2022_jp(b"\x1B(J\x5C\x1B(J\x5C", "\u{00A5}\u{00A5}");
    decode_iso_2022_jp(b"\x1B(I\x21\x1B(I\x21", "\u{FF61}\u{FF61}");
    decode_iso_2022_jp(b"\x1B$@\x54\x64\x1B$@\x54\x64", "\u{58FA}\u{58FA}");
    decode_iso_2022_jp(b"\x1B$B\x54\x64\x1B$B\x54\x64", "\u{58FA}\u{58FA}");
}

//#[test]
pub fn test_iso_2022_jp_encode() {
    // Empty
    encode_iso_2022_jp("", b"");

    // ASCII
    encode_iso_2022_jp("ab", b"ab");
    encode_iso_2022_jp("\u{1F4A9}", b"&#128169;");
    encode_iso_2022_jp("\x1B", b"&#65533;");
    encode_iso_2022_jp("\x0E", b"&#65533;");
    encode_iso_2022_jp("\x0F", b"&#65533;");

    // Roman
    encode_iso_2022_jp("a\u{00A5}b", b"a\x1B(J\x5Cb\x1B(B");
    encode_iso_2022_jp("a\u{203E}b", b"a\x1B(J\x7Eb\x1B(B");
    encode_iso_2022_jp("a\u{00A5}b\x5C", b"a\x1B(J\x5Cb\x1B(B\x5C");
    encode_iso_2022_jp("a\u{203E}b\x7E", b"a\x1B(J\x7Eb\x1B(B\x7E");
    encode_iso_2022_jp("\u{00A5}\u{1F4A9}", b"\x1B(J\x5C&#128169;\x1B(B");
    encode_iso_2022_jp("\u{00A5}\x1B", b"\x1B(J\x5C&#65533;\x1B(B");
    encode_iso_2022_jp("\u{00A5}\x0E", b"\x1B(J\x5C&#65533;\x1B(B");
    encode_iso_2022_jp("\u{00A5}\x0F", b"\x1B(J\x5C&#65533;\x1B(B");
    encode_iso_2022_jp("\u{00A5}\u{58FA}", b"\x1B(J\x5C\x1B$B\x54\x64\x1B(B");

    // Half-width Katakana
    encode_iso_2022_jp("\u{FF61}", b"\x1B$B\x21\x23\x1B(B");
    encode_iso_2022_jp("\u{FF65}", b"\x1B$B\x21\x26\x1B(B");
    encode_iso_2022_jp("\u{FF66}", b"\x1B$B\x25\x72\x1B(B");
    encode_iso_2022_jp("\u{FF70}", b"\x1B$B\x21\x3C\x1B(B");
    encode_iso_2022_jp("\u{FF9D}", b"\x1B$B\x25\x73\x1B(B");
    encode_iso_2022_jp("\u{FF9E}", b"\x1B$B\x21\x2B\x1B(B");
    encode_iso_2022_jp("\u{FF9F}", b"\x1B$B\x21\x2C\x1B(B");

    // 0208
    encode_iso_2022_jp("\u{58FA}", b"\x1B$B\x54\x64\x1B(B");
    encode_iso_2022_jp("\u{58FA}\u{250F}", b"\x1B$B\x54\x64\x28\x2E\x1B(B");
    encode_iso_2022_jp("\u{58FA}\u{1F4A9}", b"\x1B$B\x54\x64\x1B(B&#128169;");
    encode_iso_2022_jp("\u{58FA}\x1B", b"\x1B$B\x54\x64\x1B(B&#65533;");
    encode_iso_2022_jp("\u{58FA}\x0E", b"\x1B$B\x54\x64\x1B(B&#65533;");
    encode_iso_2022_jp("\u{58FA}\x0F", b"\x1B$B\x54\x64\x1B(B&#65533;");
    encode_iso_2022_jp("\u{58FA}\u{00A5}", b"\x1B$B\x54\x64\x1B(J\x5C\x1B(B");
    encode_iso_2022_jp("\u{58FA}a", b"\x1B$B\x54\x64\x1B(Ba");
}

//#[test]
pub fn test_iso_2022_jp_decode_all() {
    let input = include_bytes!("test_data/iso_2022_jp_in.txt");
    let expectation = include_str!("test_data/iso_2022_jp_in_ref.txt");
    let (cow, had_errors) = ISO_2022_JP.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_iso_2022_jp_encode_all() {
    let input = include_str!("test_data/iso_2022_jp_out.txt");
    let expectation = include_bytes!("test_data/iso_2022_jp_out_ref.txt");
    let (cow, encoding, had_errors) = ISO_2022_JP.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, ISO_2022_JP);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_iso_2022_jp_half_width_katakana_length() {
    let mut output = [0u8; 20];
    let mut decoder = ISO_2022_JP.new_decoder();
    {
        let (result, read, written) =
            decoder.decode_to_utf8_without_replacement(b"\x1B\x28\x49", &mut output, false);
        assert_eq!(result, DecoderResult::InputEmpty);
        assert_eq!(read, 3);
        assert_eq!(written, 0);
    }
    {
        let needed = decoder
            .max_utf8_buffer_length_without_replacement(1)
            .unwrap();
        let (result, read, written) =
            decoder.decode_to_utf8_without_replacement(b"\x21", &mut output[..needed], true);
        assert_eq!(result, DecoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 3);
        assert_eq!(output[0], 0xEF);
        assert_eq!(output[1], 0xBD);
        assert_eq!(output[2], 0xA1);
    }
}

//#[test]
pub fn test_iso_2022_jp_length_after_escape() {
    let mut output = [0u16; 20];
    let mut decoder = ISO_2022_JP.new_decoder();
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\x1B", &mut output, false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"A", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 2);
        assert!(had_errors);
        assert_eq!(output[0], 0xFFFD);
        assert_eq!(output[1], 0x0041);
    }
}

//#[test]
pub fn test_iso_2022_jp_encode_from_two_low_surrogates() {
    let expectation = b"&#65533;&#65533;";
    let mut output = [0u8; 40];
    let mut encoder = ISO_2022_JP.new_encoder();
    let (result, read, written, had_errors) =
        encoder.encode_from_utf16(&[0xDC00u16, 0xDEDEu16], &mut output[..], true);
    assert_eq!(result, CoderResult::InputEmpty);
    assert_eq!(read, 2);
    assert_eq!(written, expectation.len());
    assert!(had_errors);
    assert_eq!(&output[..written], expectation);
}

