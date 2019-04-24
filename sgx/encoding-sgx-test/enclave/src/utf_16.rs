//#[cfg(test)]
//mod tests {
//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use crate::testing::*;

fn decode_utf_16le(bytes: &[u8], expect: &str) {
    decode_without_padding(UTF_16LE, bytes, expect);
}

fn decode_utf_16be(bytes: &[u8], expect: &str) {
    decode_without_padding(UTF_16BE, bytes, expect);
}

fn encode_utf_16le(string: &str, expect: &[u8]) {
    encode(UTF_16LE, string, expect);
}

fn encode_utf_16be(string: &str, expect: &[u8]) {
    encode(UTF_16BE, string, expect);
}

//#[test]
pub fn test_utf_16_decode() {
    decode_utf_16le(b"", "");
    decode_utf_16be(b"", "");

    decode_utf_16le(b"\x61\x00\x62\x00", "\u{0061}\u{0062}");
    decode_utf_16be(b"\x00\x61\x00\x62", "\u{0061}\u{0062}");

    decode_utf_16le(b"\xFE\xFF\x00\x61\x00\x62", "\u{0061}\u{0062}");
    decode_utf_16be(b"\xFF\xFE\x61\x00\x62\x00", "\u{0061}\u{0062}");

    decode_utf_16le(b"\x61\x00\x62", "\u{0061}\u{FFFD}");
    decode_utf_16be(b"\x00\x61\x00", "\u{0061}\u{FFFD}");

    decode_utf_16le(b"\x3D\xD8\xA9", "\u{FFFD}");
    decode_utf_16be(b"\xD8\x3D\xDC", "\u{FFFD}");

    decode_utf_16le(b"\x3D\xD8\xA9\xDC\x03\x26", "\u{1F4A9}\u{2603}");
    decode_utf_16be(b"\xD8\x3D\xDC\xA9\x26\x03", "\u{1F4A9}\u{2603}");

    decode_utf_16le(b"\xA9\xDC\x03\x26", "\u{FFFD}\u{2603}");
    decode_utf_16be(b"\xDC\xA9\x26\x03", "\u{FFFD}\u{2603}");

    decode_utf_16le(b"\x3D\xD8\x03\x26", "\u{FFFD}\u{2603}");
    decode_utf_16be(b"\xD8\x3D\x26\x03", "\u{FFFD}\u{2603}");

    // The \xFF makes sure that the parts before and after have different alignment
    let long_le = b"\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8\xA9\xDC\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8\x00\x00\x00\x00\x00\x00\x00\x00\xA9\xDC\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8\xFF\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8\xA9\xDC\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8\x00\x00\x00\x00\x00\x00\x00\x00\xA9\xDC\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3D\xD8";
    let long_be = b"\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D\xDC\xA9\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D\x00\x00\x00\x00\x00\x00\x00\x00\xDC\xA9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D\xFF\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D\xDC\xA9\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D\x00\x00\x00\x00\x00\x00\x00\x00\xDC\xA9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xD8\x3D";
    let long_expect = "\x00\x00\x00\x00\u{1F4A9}\x00\x00\x00\x00\u{FFFD}\x00\x00\x00\x00\u{FFFD}\x00\x00\x00\x00\x00\x00\x00\x00\u{FFFD}";
    decode_utf_16le(&long_le[..long_le.len() / 2], long_expect);
    decode_utf_16be(&long_be[..long_be.len() / 2], long_expect);
    decode_utf_16le(&long_le[long_le.len() / 2 + 1..], long_expect);
    decode_utf_16be(&long_be[long_be.len() / 2 + 1..], long_expect);
}

//#[test]
pub fn test_utf_16_encode() {
    // Empty
    encode_utf_16be("", b"");
    encode_utf_16le("", b"");

    // Encodes as UTF-8
    assert_eq!(UTF_16LE.new_encoder().encoding(), UTF_8);
    assert_eq!(UTF_16BE.new_encoder().encoding(), UTF_8);
    encode_utf_16le("\u{1F4A9}\u{2603}", "\u{1F4A9}\u{2603}".as_bytes());
    encode_utf_16be("\u{1F4A9}\u{2603}", "\u{1F4A9}\u{2603}".as_bytes());
}

//#[test]
pub fn test_utf_16be_decode_one_by_one() {
    let input = b"\x00\x61\x00\xE4\x26\x03\xD8\x3D\xDC\xA9";
    let mut output = [0u16; 20];
    let mut decoder = UTF_16BE.new_decoder();
    for b in input.chunks(1) {
        assert_eq!(b.len(), 1);
        let needed = decoder.max_utf16_buffer_length(b.len()).unwrap();
        let (result, read, _, had_errors) =
            decoder.decode_to_utf16(b, &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert!(!had_errors);
    }
}

//#[test]
pub fn test_utf_16le_decode_one_by_one() {
    let input = b"\x61\x00\xE4\x00\x03\x26\x3D\xD8\xA9\xDC";
    let mut output = [0u16; 20];
    let mut decoder = UTF_16LE.new_decoder();
    for b in input.chunks(1) {
        assert_eq!(b.len(), 1);
        let needed = decoder.max_utf16_buffer_length(b.len()).unwrap();
        let (result, read, _, had_errors) =
            decoder.decode_to_utf16(b, &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert!(!had_errors);
    }
}

//#[test]
pub fn test_utf_16be_decode_three_at_a_time() {
    let input = b"\x00\xE4\x26\x03\xD8\x3D\xDC\xA9\x00\x61\x00\xE4";
    let mut output = [0u16; 20];
    let mut decoder = UTF_16BE.new_decoder();
    for b in input.chunks(3) {
        assert_eq!(b.len(), 3);
        let needed = decoder.max_utf16_buffer_length(b.len()).unwrap();
        let (result, read, _, had_errors) =
            decoder.decode_to_utf16(b, &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, b.len());
        assert!(!had_errors);
    }
}

//#[test]
pub fn test_utf_16le_decode_three_at_a_time() {
    let input = b"\xE4\x00\x03\x26\x3D\xD8\xA9\xDC\x61\x00\xE4\x00";
    let mut output = [0u16; 20];
    let mut decoder = UTF_16LE.new_decoder();
    for b in input.chunks(3) {
        assert_eq!(b.len(), 3);
        let needed = decoder.max_utf16_buffer_length(b.len()).unwrap();
        let (result, read, _, had_errors) =
            decoder.decode_to_utf16(b, &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, b.len());
        assert!(!had_errors);
    }
}

//#[test]
pub fn test_utf_16le_decode_bom_prefixed_split_byte_pair() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_16LE.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFF", &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFD", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(!had_errors);
        assert_eq!(output[0], 0xFDFF);
    }
}

//#[test]
pub fn test_utf_16be_decode_bom_prefixed_split_byte_pair() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_16BE.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFE", &mut output[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFD", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(!had_errors);
        assert_eq!(output[0], 0xFEFD);
    }
}

//#[test]
pub fn test_utf_16le_decode_bom_prefix() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_16LE.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFF", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(had_errors);
        assert_eq!(output[0], 0xFFFD);
    }
}

//#[test]
pub fn test_utf_16be_decode_bom_prefix() {
    let mut output = [0u16; 20];
    let mut decoder = UTF_16BE.new_decoder();
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, read, written, had_errors) =
            decoder.decode_to_utf16(b"\xFE", &mut output[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 1);
        assert!(had_errors);
        assert_eq!(output[0], 0xFFFD);
    }
}

//#[test]
pub fn test_utf_16le_decode_near_end() {
    let mut output = [0u8; 4];
    let mut decoder = UTF_16LE.new_decoder();
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf8(&[0x03], &mut output[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
        assert_eq!(output[0], 0x0);
    }
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf8(&[0x26, 0x03, 0x26], &mut output[..], false);
        assert_eq!(result, CoderResult::OutputFull);
        assert_eq!(read, 1);
        assert_eq!(written, 3);
        assert!(!had_errors);
        assert_eq!(output[0], 0xE2);
        assert_eq!(output[1], 0x98);
        assert_eq!(output[2], 0x83);
        assert_eq!(output[3], 0x00);
    }
}

//#[test]
pub fn test_utf_16be_decode_near_end() {
    let mut output = [0u8; 4];
    let mut decoder = UTF_16BE.new_decoder();
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf8(&[0x26], &mut output[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
        assert_eq!(read, 1);
        assert_eq!(written, 0);
        assert!(!had_errors);
        assert_eq!(output[0], 0x0);
    }
    {
        let (result, read, written, had_errors) =
            decoder.decode_to_utf8(&[0x03, 0x26, 0x03], &mut output[..], false);
        assert_eq!(result, CoderResult::OutputFull);
        assert_eq!(read, 1);
        assert_eq!(written, 3);
        assert!(!had_errors);
        assert_eq!(output[0], 0xE2);
        assert_eq!(output[1], 0x98);
        assert_eq!(output[2], 0x83);
        assert_eq!(output[3], 0x00);
    }
}
//}
