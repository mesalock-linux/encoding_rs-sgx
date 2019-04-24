// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::testing::*;
use encoding_rs::*;

// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

//#[cfg(test)]
//mod tests {
//use super::super::testing::*;
//use super::super::*;

fn decode_shift_jis(bytes: &[u8], expect: &str) {
    decode(SHIFT_JIS, bytes, expect);
}

fn encode_shift_jis(string: &str, expect: &[u8]) {
    encode(SHIFT_JIS, string, expect);
}

//#[test]
pub fn test_shift_jis_decode() {
    // Empty
    decode_shift_jis(b"", &"");

    // ASCII
    decode_shift_jis(b"\x61\x62", "\u{0061}\u{0062}");

    // Half-width
    decode_shift_jis(b"\xA1", "\u{FF61}");
    decode_shift_jis(b"\xDF", "\u{FF9F}");
    decode_shift_jis(b"\xA0", "\u{FFFD}");
    decode_shift_jis(b"\xE0", "\u{FFFD}");
    decode_shift_jis(b"\xA0+", "\u{FFFD}+");
    decode_shift_jis(b"\xE0+", "\u{FFFD}+");

    // EUDC
    decode_shift_jis(b"\xF0\x40", "\u{E000}");
    decode_shift_jis(b"\xF9\xFC", "\u{E757}");
    decode_shift_jis(b"\xEF\xFC", "\u{FFFD}");
    decode_shift_jis(b"\xFA\x40", "\u{2170}");

    // JIS 0208
    decode_shift_jis(b"\x81\x40", "\u{3000}");
    decode_shift_jis(b"\x81\x3F", "\u{FFFD}?");
    decode_shift_jis(b"\xEE\xFC", "\u{FF02}");
    decode_shift_jis(b"\xEE\xFD", "\u{FFFD}");
    decode_shift_jis(b"\xFA\x40", "\u{2170}");
    decode_shift_jis(b"\xFA\x3F", "\u{FFFD}?");
    decode_shift_jis(b"\xFC\x4B", "\u{9ED1}");
    decode_shift_jis(b"\xFC\x4C", "\u{FFFD}L");
    //
}

//#[test]
pub fn test_shift_jis_encode() {
    // Empty
    encode_shift_jis("", b"");

    // ASCII
    encode_shift_jis("\u{0061}\u{0062}", b"\x61\x62");

    // Exceptional code points
    encode_shift_jis("\u{0080}", b"\x80");
    encode_shift_jis("\u{00A5}", b"\x5C");
    encode_shift_jis("\u{203E}", b"\x7E");
    encode_shift_jis("\u{2212}", b"\x81\x7C");

    // Half-width
    encode_shift_jis("\u{FF61}", b"\xA1");
    encode_shift_jis("\u{FF9F}", b"\xDF");

    // EUDC
    encode_shift_jis("\u{E000}", b"&#57344;");
    encode_shift_jis("\u{E757}", b"&#59223;");

    // JIS 0212
    encode_shift_jis("\u{02D8}", b"&#728;");

    // JIS 0208
    encode_shift_jis("\u{3000}", b"\x81\x40");
    encode_shift_jis("\u{FF02}", b"\xFA\x57");
    encode_shift_jis("\u{2170}", b"\xFA\x40");
    encode_shift_jis("\u{9ED1}", b"\xFC\x4B");
}

//#[test]
pub fn test_shift_jis_decode_all() {
    let input = include_bytes!("test_data/shift_jis_in.txt");
    let expectation = include_str!("test_data/shift_jis_in_ref.txt");
    let (cow, had_errors) = SHIFT_JIS.decode_without_bom_handling(input);
    assert!(had_errors, "Should have had errors.");
    assert_eq!(&cow[..], expectation);
}

//#[test]
pub fn test_shift_jis_encode_all() {
    let input = include_str!("test_data/shift_jis_out.txt");
    let expectation = include_bytes!("test_data/shift_jis_out_ref.txt");
    let (cow, encoding, had_errors) = SHIFT_JIS.encode(input);
    assert!(!had_errors, "Should not have had errors.");
    assert_eq!(encoding, SHIFT_JIS);
    assert_eq!(&cow[..], &expectation[..]);
}

//#[test]
pub fn test_shift_jis_half_width_katakana_length() {
    let mut output = [0u8; 20];
    let mut decoder = SHIFT_JIS.new_decoder();
    {
        let needed = decoder
            .max_utf8_buffer_length_without_replacement(1)
            .unwrap();
        let (result, read, written) =
            decoder.decode_to_utf8_without_replacement(b"\xA1", &mut output[..needed], true);
        assert_eq!(result, DecoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 3);
        assert_eq!(output[0], 0xEF);
        assert_eq!(output[1], 0xBD);
        assert_eq!(output[2], 0xA1);
    }
}
//}
