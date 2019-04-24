//use super::super::testing::*;
//use super::super::*;
use encoding_rs::*;
use encoding_rs::data;
use crate::testing::*;

//#[test]
pub fn test_windows_1255_ca() {
    decode(WINDOWS_1255, b"\xCA", "\u{05BA}");
    encode(WINDOWS_1255, "\u{05BA}", b"\xCA");
}

//#[test]
pub fn test_ascii_punctuation() {
    let bytes = b"\xC1\xF5\xF4\xFC \xE5\xDF\xED\xE1\xE9 \xDD\xED\xE1 \xF4\xE5\xF3\xF4. \xC1\xF5\xF4\xFC \xE5\xDF\xED\xE1\xE9 \xDD\xED\xE1 \xF4\xE5\xF3\xF4.";
    let characters = "\u{0391}\u{03C5}\u{03C4}\u{03CC} \
                      \u{03B5}\u{03AF}\u{03BD}\u{03B1}\u{03B9} \u{03AD}\u{03BD}\u{03B1} \
                      \u{03C4}\u{03B5}\u{03C3}\u{03C4}. \u{0391}\u{03C5}\u{03C4}\u{03CC} \
                      \u{03B5}\u{03AF}\u{03BD}\u{03B1}\u{03B9} \u{03AD}\u{03BD}\u{03B1} \
                      \u{03C4}\u{03B5}\u{03C3}\u{03C4}.";
    decode(WINDOWS_1253, bytes, characters);
    encode(WINDOWS_1253, characters, bytes);
}

//#[test]
pub fn test_decode_malformed() {
    decode(
        WINDOWS_1253,
        b"\xC1\xF5\xD2\xF4\xFC",
        "\u{0391}\u{03C5}\u{FFFD}\u{03C4}\u{03CC}",
    );
}

//#[test]
pub fn test_encode_unmappables() {
    encode(
        WINDOWS_1253,
        "\u{0391}\u{03C5}\u{2603}\u{03C4}\u{03CC}",
        b"\xC1\xF5&#9731;\xF4\xFC",
    );
    encode(
        WINDOWS_1253,
        "\u{0391}\u{03C5}\u{1F4A9}\u{03C4}\u{03CC}",
        b"\xC1\xF5&#128169;\xF4\xFC",
    );
}

//#[test]
pub fn test_encode_unpaired_surrogates() {
    encode_from_utf16(
        WINDOWS_1253,
        &[0x0391u16, 0x03C5u16, 0xDCA9u16, 0x03C4u16, 0x03CCu16],
        b"\xC1\xF5&#65533;\xF4\xFC",
    );
    encode_from_utf16(
        WINDOWS_1253,
        &[0x0391u16, 0x03C5u16, 0xD83Du16, 0x03C4u16, 0x03CCu16],
        b"\xC1\xF5&#65533;\xF4\xFC",
    );
    encode_from_utf16(
        WINDOWS_1253,
        &[0x0391u16, 0x03C5u16, 0x03C4u16, 0x03CCu16, 0xD83Du16],
        b"\xC1\xF5\xF4\xFC&#65533;",
    );
}

pub const HIGH_BYTES: &'static [u8; 128] = &[
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E,
    0x8F, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D,
    0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC,
    0xAD, 0xAE, 0xAF, 0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB,
    0xBC, 0xBD, 0xBE, 0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA,
    0xCB, 0xCC, 0xCD, 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9,
    0xDA, 0xDB, 0xDC, 0xDD, 0xDE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8,
    0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
    0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
];

fn decode_single_byte(encoding: &'static Encoding, data: &'static [u16; 128]) {
    let mut with_replacement = [0u16; 128];
    let mut it = data.iter().enumerate();
    loop {
        match it.next() {
            Some((i, code_point)) => {
                if *code_point == 0 {
                    with_replacement[i] = 0xFFFD;
                } else {
                    with_replacement[i] = *code_point;
                }
            }
            None => {
                break;
            }
        }
    }

    decode_to_utf16(encoding, HIGH_BYTES, &with_replacement[..]);
}

fn encode_single_byte(encoding: &'static Encoding, data: &'static [u16; 128]) {
    let mut with_zeros = [0u8; 128];
    let mut it = data.iter().enumerate();
    loop {
        match it.next() {
            Some((i, code_point)) => {
                if *code_point == 0 {
                    with_zeros[i] = 0;
                } else {
                    with_zeros[i] = HIGH_BYTES[i];
                }
            }
            None => {
                break;
            }
        }
    }

    encode_from_utf16(encoding, data, &with_zeros[..]);
}

//#[test]
pub fn test_single_byte_from_two_low_surrogates() {
    let expectation = b"&#65533;&#65533;";
    let mut output = [0u8; 40];
    let mut encoder = WINDOWS_1253.new_encoder();
    let (result, read, written, had_errors) =
        encoder.encode_from_utf16(&[0xDC00u16, 0xDEDEu16], &mut output[..], true);
    assert_eq!(result, CoderResult::InputEmpty);
    assert_eq!(read, 2);
    assert_eq!(written, expectation.len());
    assert!(had_errors);
    assert_eq!(&output[..written], expectation);
}

// These tests are so self-referential that they are pretty useless.

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate-encoding-data.py

//#[test]
pub fn test_single_byte_decode() {
    decode_single_byte(IBM866, &data::SINGLE_BYTE_DATA.ibm866);
    decode_single_byte(ISO_8859_10, &data::SINGLE_BYTE_DATA.iso_8859_10);
    decode_single_byte(ISO_8859_13, &data::SINGLE_BYTE_DATA.iso_8859_13);
    decode_single_byte(ISO_8859_14, &data::SINGLE_BYTE_DATA.iso_8859_14);
    decode_single_byte(ISO_8859_15, &data::SINGLE_BYTE_DATA.iso_8859_15);
    decode_single_byte(ISO_8859_16, &data::SINGLE_BYTE_DATA.iso_8859_16);
    decode_single_byte(ISO_8859_2, &data::SINGLE_BYTE_DATA.iso_8859_2);
    decode_single_byte(ISO_8859_3, &data::SINGLE_BYTE_DATA.iso_8859_3);
    decode_single_byte(ISO_8859_4, &data::SINGLE_BYTE_DATA.iso_8859_4);
    decode_single_byte(ISO_8859_5, &data::SINGLE_BYTE_DATA.iso_8859_5);
    decode_single_byte(ISO_8859_6, &data::SINGLE_BYTE_DATA.iso_8859_6);
    decode_single_byte(ISO_8859_7, &data::SINGLE_BYTE_DATA.iso_8859_7);
    decode_single_byte(ISO_8859_8, &data::SINGLE_BYTE_DATA.iso_8859_8);
    decode_single_byte(KOI8_R, &data::SINGLE_BYTE_DATA.koi8_r);
    decode_single_byte(KOI8_U, &data::SINGLE_BYTE_DATA.koi8_u);
    decode_single_byte(MACINTOSH, &data::SINGLE_BYTE_DATA.macintosh);
    decode_single_byte(WINDOWS_1250, &data::SINGLE_BYTE_DATA.windows_1250);
    decode_single_byte(WINDOWS_1251, &data::SINGLE_BYTE_DATA.windows_1251);
    decode_single_byte(WINDOWS_1252, &data::SINGLE_BYTE_DATA.windows_1252);
    decode_single_byte(WINDOWS_1253, &data::SINGLE_BYTE_DATA.windows_1253);
    decode_single_byte(WINDOWS_1254, &data::SINGLE_BYTE_DATA.windows_1254);
    decode_single_byte(WINDOWS_1255, &data::SINGLE_BYTE_DATA.windows_1255);
    decode_single_byte(WINDOWS_1256, &data::SINGLE_BYTE_DATA.windows_1256);
    decode_single_byte(WINDOWS_1257, &data::SINGLE_BYTE_DATA.windows_1257);
    decode_single_byte(WINDOWS_1258, &data::SINGLE_BYTE_DATA.windows_1258);
    decode_single_byte(WINDOWS_874, &data::SINGLE_BYTE_DATA.windows_874);
    decode_single_byte(X_MAC_CYRILLIC, &data::SINGLE_BYTE_DATA.x_mac_cyrillic);
}

//#[test]
pub fn test_single_byte_encode() {
    encode_single_byte(IBM866, &data::SINGLE_BYTE_DATA.ibm866);
    encode_single_byte(ISO_8859_10, &data::SINGLE_BYTE_DATA.iso_8859_10);
    encode_single_byte(ISO_8859_13, &data::SINGLE_BYTE_DATA.iso_8859_13);
    encode_single_byte(ISO_8859_14, &data::SINGLE_BYTE_DATA.iso_8859_14);
    encode_single_byte(ISO_8859_15, &data::SINGLE_BYTE_DATA.iso_8859_15);
    encode_single_byte(ISO_8859_16, &data::SINGLE_BYTE_DATA.iso_8859_16);
    encode_single_byte(ISO_8859_2, &data::SINGLE_BYTE_DATA.iso_8859_2);
    encode_single_byte(ISO_8859_3, &data::SINGLE_BYTE_DATA.iso_8859_3);
    encode_single_byte(ISO_8859_4, &data::SINGLE_BYTE_DATA.iso_8859_4);
    encode_single_byte(ISO_8859_5, &data::SINGLE_BYTE_DATA.iso_8859_5);
    encode_single_byte(ISO_8859_6, &data::SINGLE_BYTE_DATA.iso_8859_6);
    encode_single_byte(ISO_8859_7, &data::SINGLE_BYTE_DATA.iso_8859_7);
    encode_single_byte(ISO_8859_8, &data::SINGLE_BYTE_DATA.iso_8859_8);
    encode_single_byte(KOI8_R, &data::SINGLE_BYTE_DATA.koi8_r);
    encode_single_byte(KOI8_U, &data::SINGLE_BYTE_DATA.koi8_u);
    encode_single_byte(MACINTOSH, &data::SINGLE_BYTE_DATA.macintosh);
    encode_single_byte(WINDOWS_1250, &data::SINGLE_BYTE_DATA.windows_1250);
    encode_single_byte(WINDOWS_1251, &data::SINGLE_BYTE_DATA.windows_1251);
    encode_single_byte(WINDOWS_1252, &data::SINGLE_BYTE_DATA.windows_1252);
    encode_single_byte(WINDOWS_1253, &data::SINGLE_BYTE_DATA.windows_1253);
    encode_single_byte(WINDOWS_1254, &data::SINGLE_BYTE_DATA.windows_1254);
    encode_single_byte(WINDOWS_1255, &data::SINGLE_BYTE_DATA.windows_1255);
    encode_single_byte(WINDOWS_1256, &data::SINGLE_BYTE_DATA.windows_1256);
    encode_single_byte(WINDOWS_1257, &data::SINGLE_BYTE_DATA.windows_1257);
    encode_single_byte(WINDOWS_1258, &data::SINGLE_BYTE_DATA.windows_1258);
    encode_single_byte(WINDOWS_874, &data::SINGLE_BYTE_DATA.windows_874);
    encode_single_byte(X_MAC_CYRILLIC, &data::SINGLE_BYTE_DATA.x_mac_cyrillic);
}
// END GENERATED CODE

