// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

//use super::super::testing::*;
//use super::super::*;

//    fn decode_utf8_to_utf16(bytes: &[u8], expect: &[u16]) {
//        decode_to_utf16_without_replacement(UTF_8, bytes, expect);
//    }

use std::prelude::v1::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_utf8_to_utf8(bytes: &[u8], expect: &str) {
    decode_to_utf8(UTF_8, bytes, expect);
}

fn decode_valid_utf8(string: &str) {
    decode_utf8_to_utf8(string.as_bytes(), string);
}

fn encode_utf8_from_utf16(string: &[u16], expect: &[u8]) {
    encode_from_utf16(UTF_8, string, expect);
}

fn encode_utf8_from_utf8(string: &str, expect: &[u8]) {
    encode_from_utf8(UTF_8, string, expect);
}

fn encode_utf8_from_utf16_with_output_limit(
    string: &[u16],
    expect: &str,
    limit: usize,
    expect_result: EncoderResult,
) {
    let mut dst = Vec::new();
    {
        dst.resize(limit, 0u8);
        let mut encoder = UTF_8.new_encoder();
        let (result, read, written) =
            encoder.encode_from_utf16_without_replacement(string, &mut dst, false);
        assert_eq!(result, expect_result);
        if expect_result == EncoderResult::InputEmpty {
            assert_eq!(read, string.len());
        }
        assert_eq!(&dst[..written], expect.as_bytes());
    }
    {
        dst.resize(64, 0u8);
        for (i, elem) in dst.iter_mut().enumerate() {
            *elem = i as u8;
        }
        let mut encoder = UTF_8.new_encoder();
        let (_, _, mut j) =
            encoder.encode_from_utf16_without_replacement(string, &mut dst, false);
        while j < dst.len() {
            assert_eq!(usize::from(dst[j]), j);
            j += 1;
        }
    }
}

//#[test]
pub fn test_utf8_decode() {
    // Empty
    decode_valid_utf8("");
    // ASCII
    decode_valid_utf8("ab");
    // Low BMP
    decode_valid_utf8("a\u{E4}Z");
    // High BMP
    decode_valid_utf8("a\u{2603}Z");
    // Astral
    decode_valid_utf8("a\u{1F4A9}Z");
    // Low BMP with last byte missing
    decode_utf8_to_utf8(b"a\xC3Z", "a\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xC3", "a\u{FFFD}");
    // High BMP with last byte missing
    decode_utf8_to_utf8(b"a\xE2\x98Z", "a\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xE2\x98", "a\u{FFFD}");
    // Astral with last byte missing
    decode_utf8_to_utf8(b"a\xF0\x9F\x92Z", "a\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xF0\x9F\x92", "a\u{FFFD}");
    // Lone highest continuation
    decode_utf8_to_utf8(b"a\xBFZ", "a\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xBF", "a\u{FFFD}");
    // Two lone highest continuations
    decode_utf8_to_utf8(b"a\xBF\xBFZ", "a\u{FFFD}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xBF\xBF", "a\u{FFFD}\u{FFFD}");
    // Low BMP followed by lowest lone continuation
    decode_utf8_to_utf8(b"a\xC3\xA4\x80Z", "a\u{E4}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xC3\xA4\x80", "a\u{E4}\u{FFFD}");
    // Low BMP followed by highest lone continuation
    decode_utf8_to_utf8(b"a\xC3\xA4\xBFZ", "a\u{E4}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xC3\xA4\xBF", "a\u{E4}\u{FFFD}");
    // High BMP followed by lowest lone continuation
    decode_utf8_to_utf8(b"a\xE2\x98\x83\x80Z", "a\u{2603}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xE2\x98\x83\x80", "a\u{2603}\u{FFFD}");
    // High BMP followed by highest lone continuation
    decode_utf8_to_utf8(b"a\xE2\x98\x83\xBFZ", "a\u{2603}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xE2\x98\x83\xBF", "a\u{2603}\u{FFFD}");
    // Astral followed by lowest lone continuation
    decode_utf8_to_utf8(b"a\xF0\x9F\x92\xA9\x80Z", "a\u{1F4A9}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xF0\x9F\x92\xA9\x80", "a\u{1F4A9}\u{FFFD}");
    // Astral followed by highest lone continuation
    decode_utf8_to_utf8(b"a\xF0\x9F\x92\xA9\xBFZ", "a\u{1F4A9}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\xF0\x9F\x92\xA9\xBF", "a\u{1F4A9}\u{FFFD}");

    // Boundary conditions
    // Lowest single-byte
    decode_valid_utf8("Z\x00");
    decode_valid_utf8("Z\x00Z");
    // Lowest single-byte as two-byte overlong sequence
    decode_utf8_to_utf8(b"a\xC0\x80", "a\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xC0\x80Z", "a\u{FFFD}\u{FFFD}Z");
    // Lowest single-byte as three-byte overlong sequence
    decode_utf8_to_utf8(b"a\xE0\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xE0\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lowest single-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x80\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x80\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // One below lowest single-byte
    decode_utf8_to_utf8(b"a\xFF", "a\u{FFFD}");
    decode_utf8_to_utf8(b"a\xFFZ", "a\u{FFFD}Z");
    // Highest single-byte
    decode_valid_utf8("a\x7F");
    decode_valid_utf8("a\x7FZ");
    // Highest single-byte as two-byte overlong sequence
    decode_utf8_to_utf8(b"a\xC1\xBF", "a\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xC1\xBFZ", "a\u{FFFD}\u{FFFD}Z");
    // Highest single-byte as three-byte overlong sequence
    decode_utf8_to_utf8(b"a\xE0\x81\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xE0\x81\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Highest single-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x80\x81\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x80\x81\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // One past highest single byte (also lone continuation)
    decode_utf8_to_utf8(b"a\x80Z", "a\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\x80", "a\u{FFFD}");
    // Two lone continuations
    decode_utf8_to_utf8(b"a\x80\x80Z", "a\u{FFFD}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\x80\x80", "a\u{FFFD}\u{FFFD}");
    // Three lone continuations
    decode_utf8_to_utf8(b"a\x80\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\x80\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    // Four lone continuations
    decode_utf8_to_utf8(b"a\x80\x80\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    decode_utf8_to_utf8(b"a\x80\x80\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    // Lowest two-byte
    decode_utf8_to_utf8(b"a\xC2\x80", "a\u{0080}");
    decode_utf8_to_utf8(b"a\xC2\x80Z", "a\u{0080}Z");
    // Lowest two-byte as three-byte overlong sequence
    decode_utf8_to_utf8(b"a\xE0\x82\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xE0\x82\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lowest two-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x80\x82\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x80\x82\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lead one below lowest two-byte
    decode_utf8_to_utf8(b"a\xC1\x80", "a\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xC1\x80Z", "a\u{FFFD}\u{FFFD}Z");
    // Trail one below lowest two-byte
    decode_utf8_to_utf8(b"a\xC2\x7F", "a\u{FFFD}\u{007F}");
    decode_utf8_to_utf8(b"a\xC2\x7FZ", "a\u{FFFD}\u{007F}Z");
    // Highest two-byte
    decode_utf8_to_utf8(b"a\xDF\xBF", "a\u{07FF}");
    decode_utf8_to_utf8(b"a\xDF\xBFZ", "a\u{07FF}Z");
    // Highest two-byte as three-byte overlong sequence
    decode_utf8_to_utf8(b"a\xE0\x9F\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xE0\x9F\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Highest two-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x80\x9F\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x80\x9F\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lowest three-byte
    decode_utf8_to_utf8(b"a\xE0\xA0\x80", "a\u{0800}");
    decode_utf8_to_utf8(b"a\xE0\xA0\x80Z", "a\u{0800}Z");
    // Lowest three-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x80\xA0\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x80\xA0\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Highest below surrogates
    decode_utf8_to_utf8(b"a\xED\x9F\xBF", "a\u{D7FF}");
    decode_utf8_to_utf8(b"a\xED\x9F\xBFZ", "a\u{D7FF}Z");
    // Highest below surrogates as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x8D\x9F\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x8D\x9F\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // First surrogate
    decode_utf8_to_utf8(b"a\xED\xA0\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xED\xA0\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // First surrogate as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x8D\xA0\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x8D\xA0\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Last surrogate
    decode_utf8_to_utf8(b"a\xED\xBF\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xED\xBF\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Last surrogate as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x8D\xBF\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x8D\xBF\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lowest above surrogates
    decode_utf8_to_utf8(b"a\xEE\x80\x80", "a\u{E000}");
    decode_utf8_to_utf8(b"a\xEE\x80\x80Z", "a\u{E000}Z");
    // Lowest above surrogates as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x8E\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x8E\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Highest three-byte
    decode_utf8_to_utf8(b"a\xEF\xBF\xBF", "a\u{FFFF}");
    decode_utf8_to_utf8(b"a\xEF\xBF\xBFZ", "a\u{FFFF}Z");
    // Highest three-byte as four-byte overlong sequence
    decode_utf8_to_utf8(b"a\xF0\x8F\xBF\xBF", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF0\x8F\xBF\xBFZ", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");
    // Lowest four-byte
    decode_utf8_to_utf8(b"a\xF0\x90\x80\x80", "a\u{10000}");
    decode_utf8_to_utf8(b"a\xF0\x90\x80\x80Z", "a\u{10000}Z");
    // Highest four-byte
    decode_utf8_to_utf8(b"a\xF4\x8F\xBF\xBF", "a\u{10FFFF}");
    decode_utf8_to_utf8(b"a\xF4\x8F\xBF\xBFZ", "a\u{10FFFF}Z");
    // One past highest four-byte
    decode_utf8_to_utf8(b"a\xF4\x90\x80\x80", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF4\x90\x80\x80Z", "a\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}Z");

    // Highest four-byte with last byte replaced with 0xFF
    decode_utf8_to_utf8(b"a\xF4\x8F\xBF\xFF", "a\u{FFFD}\u{FFFD}");
    decode_utf8_to_utf8(b"a\xF4\x8F\xBF\xFFZ", "a\u{FFFD}\u{FFFD}Z");
}

//#[test]
pub fn test_utf8_encode() {
    // Empty
    encode_utf8_from_utf16(&[], b"");
    encode_utf8_from_utf8("", b"");

    encode_utf8_from_utf16(&[0x0000], "\u{0000}".as_bytes());
    encode_utf8_from_utf16(&[0x007F], "\u{007F}".as_bytes());
    encode_utf8_from_utf16(&[0x0080], "\u{0080}".as_bytes());
    encode_utf8_from_utf16(&[0x07FF], "\u{07FF}".as_bytes());
    encode_utf8_from_utf16(&[0x0800], "\u{0800}".as_bytes());
    encode_utf8_from_utf16(&[0xD7FF], "\u{D7FF}".as_bytes());
    encode_utf8_from_utf16(&[0xD800], "\u{FFFD}".as_bytes());
    encode_utf8_from_utf16(&[0xD800, 0x0062], "\u{FFFD}\u{0062}".as_bytes());
    encode_utf8_from_utf16(&[0xDFFF], "\u{FFFD}".as_bytes());
    encode_utf8_from_utf16(&[0xDFFF, 0x0062], "\u{FFFD}\u{0062}".as_bytes());
    encode_utf8_from_utf16(&[0xE000], "\u{E000}".as_bytes());
    encode_utf8_from_utf16(&[0xFFFF], "\u{FFFF}".as_bytes());
    encode_utf8_from_utf16(&[0xD800, 0xDC00], "\u{10000}".as_bytes());
    encode_utf8_from_utf16(&[0xDBFF, 0xDFFF], "\u{10FFFF}".as_bytes());
    encode_utf8_from_utf16(&[0xDC00, 0xDEDE], "\u{FFFD}\u{FFFD}".as_bytes());
}

//#[test]
pub fn test_encode_utf8_from_utf16_with_output_limit() {
    encode_utf8_from_utf16_with_output_limit(&[0x0062], "\u{62}", 1, EncoderResult::InputEmpty);
    encode_utf8_from_utf16_with_output_limit(&[0x00A7], "\u{A7}", 2, EncoderResult::InputEmpty);
    encode_utf8_from_utf16_with_output_limit(
        &[0x2603],
        "\u{2603}",
        3,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDCA9],
        "\u{1F4A9}",
        4,
        EncoderResult::InputEmpty,
    );

    encode_utf8_from_utf16_with_output_limit(&[0x00A7], "", 1, EncoderResult::OutputFull);
    encode_utf8_from_utf16_with_output_limit(&[0x2603], "", 2, EncoderResult::OutputFull);
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDCA9],
        "",
        3,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x0062],
        "\u{63}\u{62}",
        2,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00A7],
        "\u{63}\u{A7}",
        3,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x2603],
        "\u{63}\u{2603}",
        4,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0xD83D, 0xDCA9],
        "\u{63}\u{1F4A9}",
        5,
        EncoderResult::InputEmpty,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00A7],
        "\u{63}",
        2,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x2603],
        "\u{63}",
        3,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0xD83D, 0xDCA9],
        "\u{63}",
        4,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0x0062],
        "\u{B6}\u{62}",
        3,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0x00A7],
        "\u{B6}\u{A7}",
        4,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0x2603],
        "\u{B6}\u{2603}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0xD83D, 0xDCA9],
        "\u{B6}\u{1F4A9}",
        6,
        EncoderResult::InputEmpty,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0x00A7],
        "\u{B6}",
        3,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0x2603],
        "\u{B6}",
        4,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x00B6, 0xD83D, 0xDCA9],
        "\u{B6}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062],
        "\u{263A}\u{62}",
        4,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x00A7],
        "\u{263A}\u{A7}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x2603],
        "\u{263A}\u{2603}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xD83D, 0xDCA9],
        "\u{263A}\u{1F4A9}",
        7,
        EncoderResult::InputEmpty,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x00A7],
        "\u{263A}",
        4,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x2603],
        "\u{263A}",
        5,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xD83D, 0xDCA9],
        "\u{263A}",
        6,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0x0062],
        "\u{1F60E}\u{62}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0x00A7],
        "\u{1F60E}\u{A7}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0x2603],
        "\u{1F60E}\u{2603}",
        7,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0xD83D, 0xDCA9],
        "\u{1F60E}\u{1F4A9}",
        8,
        EncoderResult::InputEmpty,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0x00A7],
        "\u{1F60E}",
        5,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0x2603],
        "\u{1F60E}",
        6,
        EncoderResult::OutputFull,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0xD83D, 0xDE0E, 0xD83D, 0xDCA9],
        "\u{1F60E}",
        7,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x0062],
        "\u{63}\u{B6}\u{62}\u{62}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x0062],
        "\u{63}\u{B6}\u{62}",
        4,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x0062, 0x0062],
        "\u{63}\u{B6}\u{62}\u{62}\u{62}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x0062, 0x0062],
        "\u{63}\u{B6}\u{62}\u{62}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x0062],
        "\u{263A}\u{62}\u{62}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x0062],
        "\u{263A}\u{62}",
        4,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x0062, 0x0062],
        "\u{263A}\u{62}\u{62}\u{62}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x0062, 0x0062],
        "\u{263A}\u{62}\u{62}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x00A7],
        "\u{63}\u{B6}\u{A7}",
        5,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x00A7],
        "\u{63}\u{B6}",
        4,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x00A7, 0x0062],
        "\u{63}\u{B6}\u{A7}\u{62}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x00A7, 0x0062],
        "\u{63}\u{B6}\u{A7}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x00A7, 0x0062],
        "\u{263A}\u{A7}\u{62}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x00A7, 0x0062],
        "\u{263A}\u{A7}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x00A7],
        "\u{63}\u{B6}\u{62}\u{A7}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x0062, 0x00A7],
        "\u{63}\u{B6}\u{62}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x00A7],
        "\u{263A}\u{62}\u{A7}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x0062, 0x00A7],
        "\u{263A}\u{62}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x2603],
        "\u{63}\u{B6}\u{2603}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0x2603],
        "\u{63}\u{B6}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x2603],
        "\u{263A}\u{2603}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0x2603],
        "\u{263A}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0xD83D],
        "\u{63}\u{B6}\u{FFFD}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0xD83D],
        "\u{63}\u{B6}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xD83D],
        "\u{263A}\u{FFFD}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xD83D],
        "\u{263A}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0xDCA9],
        "\u{63}\u{B6}\u{FFFD}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x0063, 0x00B6, 0xDCA9],
        "\u{63}\u{B6}",
        5,
        EncoderResult::OutputFull,
    );

    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xDCA9],
        "\u{263A}\u{FFFD}",
        6,
        EncoderResult::InputEmpty,
    );
    encode_utf8_from_utf16_with_output_limit(
        &[0x263A, 0xDCA9],
        "\u{263A}",
        5,
        EncoderResult::OutputFull,
    );
}

//#[test]
pub fn test_utf8_max_length_from_utf16() {
    let mut encoder = UTF_8.new_encoder();
    let mut output = [0u8; 13];
    let input = &[0x2C9Fu16, 0x2CA9u16, 0x2CA3u16, 0x2C9Fu16];
    let needed = encoder
        .max_buffer_length_from_utf16_without_replacement(input.len())
        .unwrap();
    let (result, _, _) =
        encoder.encode_from_utf16_without_replacement(input, &mut output[..needed], true);
    assert_eq!(result, EncoderResult::InputEmpty);
}

//#[test]
pub fn test_decode_bom_prefixed_split_byte_triple() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_8.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xEF", &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xBF", &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xBE", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(!had_errors);
        assert_eq!(output[0], 0xFFFE);
    }
}

//#[test]
pub fn test_decode_bom_prefixed_split_byte_pair() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_8.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xEF", &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xBC", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(had_errors);
        assert_eq!(output[0], 0xFFFD);
    }
}

//#[test]
pub fn test_decode_bom_prefix() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_8.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xEF", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(had_errors);
        assert_eq!(output[0], 0xFFFD);
    }
}

//#[test]
pub fn test_tail() {
    let mut output = [0u16; 1];
    let mut decoder = UTF_8.new_decoder_without_bom_handling();
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16("\u{E4}a".as_bytes(), &mut output[..], false);
        assert_eq!(result, CoderResult::OutputFull);
        assert_eq!(read, 2);
        assert_eq!(written, 1);
        assert!(!had_errors);
        assert_eq!(output[0], 0x00E4);
    }
}
