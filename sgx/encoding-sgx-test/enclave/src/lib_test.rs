use encoding_rs::*;
use std::prelude::v1::*;
use std::borrow::Cow;

fn sniff_to_utf16(
    initial_encoding: &'static Encoding,
    expected_encoding: &'static Encoding,
    bytes: &[u8],
    expect: &[u16],
    breaks: &[usize],
) {
    let mut decoder = initial_encoding.new_decoder();

    let mut dest: Vec<u16> =
        Vec::with_capacity(decoder.max_utf16_buffer_length(bytes.len()).unwrap());
    let capacity = dest.capacity();
    dest.resize(capacity, 0u16);

    let mut total_written = 0usize;
    let mut start = 0usize;
    for br in breaks {
        let (result, read, written, _) =
            decoder.decode_to_utf16(&bytes[start..*br], &mut dest[total_written..], false);
        total_written += written;
        assert_eq!(read, *br - start);
        match result {
            CoderResult::InputEmpty => {}
            CoderResult::OutputFull => {
                unreachable!();
            }
        }
        start = *br;
    }
    let (result, read, written, _) =
        decoder.decode_to_utf16(&bytes[start..], &mut dest[total_written..], true);
    total_written += written;
    match result {
        CoderResult::InputEmpty => {}
        CoderResult::OutputFull => {
            unreachable!();
        }
    }
    assert_eq!(read, bytes.len() - start);
    assert_eq!(total_written, expect.len());
    assert_eq!(&dest[..total_written], expect);
    assert_eq!(decoder.encoding(), expected_encoding);
}

// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

//#[test]
pub fn test_bom_sniffing() {
    // ASCII
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[],
    );
    // UTF-8
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[1],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[2],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[3],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[4],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[2, 3],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[1, 2],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[1, 3],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        UTF_8,
        b"\xEF\xBB\xBF\x61\x62",
        &[0x0061u16, 0x0062u16],
        &[1, 2, 3, 4],
    );
    sniff_to_utf16(WINDOWS_1252, UTF_8, b"\xEF\xBB\xBF", &[], &[]);
    // Not UTF-8
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\xBB\x61\x62",
        &[0x00EFu16, 0x00BBu16, 0x0061u16, 0x0062u16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\xBB\x61\x62",
        &[0x00EFu16, 0x00BBu16, 0x0061u16, 0x0062u16],
        &[1],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\x61\x62",
        &[0x00EFu16, 0x0061u16, 0x0062u16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\x61\x62",
        &[0x00EFu16, 0x0061u16, 0x0062u16],
        &[1],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\xBB",
        &[0x00EFu16, 0x00BBu16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xEF\xBB",
        &[0x00EFu16, 0x00BBu16],
        &[1],
    );
    sniff_to_utf16(WINDOWS_1252, WINDOWS_1252, b"\xEF", &[0x00EFu16], &[]);
    // Not UTF-16
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xFE\x61\x62",
        &[0x00FEu16, 0x0061u16, 0x0062u16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xFE\x61\x62",
        &[0x00FEu16, 0x0061u16, 0x0062u16],
        &[1],
    );
    sniff_to_utf16(WINDOWS_1252, WINDOWS_1252, b"\xFE", &[0x00FEu16], &[]);
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xFF\x61\x62",
        &[0x00FFu16, 0x0061u16, 0x0062u16],
        &[],
    );
    sniff_to_utf16(
        WINDOWS_1252,
        WINDOWS_1252,
        b"\xFF\x61\x62",
        &[0x00FFu16, 0x0061u16, 0x0062u16],
        &[1],
    );
    sniff_to_utf16(WINDOWS_1252, WINDOWS_1252, b"\xFF", &[0x00FFu16], &[]);
    // UTF-16
    sniff_to_utf16(WINDOWS_1252, UTF_16BE, b"\xFE\xFF", &[], &[]);
    sniff_to_utf16(WINDOWS_1252, UTF_16BE, b"\xFE\xFF", &[], &[1]);
    sniff_to_utf16(WINDOWS_1252, UTF_16LE, b"\xFF\xFE", &[], &[]);
    sniff_to_utf16(WINDOWS_1252, UTF_16LE, b"\xFF\xFE", &[], &[1]);
}

//#[test]
pub fn test_output_encoding() {
    assert_eq!(REPLACEMENT.output_encoding(), UTF_8);
    assert_eq!(UTF_16BE.output_encoding(), UTF_8);
    assert_eq!(UTF_16LE.output_encoding(), UTF_8);
    assert_eq!(UTF_8.output_encoding(), UTF_8);
    assert_eq!(WINDOWS_1252.output_encoding(), WINDOWS_1252);
    assert_eq!(REPLACEMENT.new_encoder().encoding(), UTF_8);
    assert_eq!(UTF_16BE.new_encoder().encoding(), UTF_8);
    assert_eq!(UTF_16LE.new_encoder().encoding(), UTF_8);
    assert_eq!(UTF_8.new_encoder().encoding(), UTF_8);
    assert_eq!(WINDOWS_1252.new_encoder().encoding(), WINDOWS_1252);
}

//#[test]
pub fn test_label_resolution() {
    assert_eq!(Encoding::for_label(b"utf-8"), Some(UTF_8));
    assert_eq!(Encoding::for_label(b"UTF-8"), Some(UTF_8));
    assert_eq!(
        Encoding::for_label(b" \t \n \x0C \n utf-8 \r \n \t \x0C "),
        Some(UTF_8)
    );
    assert_eq!(Encoding::for_label(b"utf-8 _"), None);
    assert_eq!(Encoding::for_label(b"bogus"), None);
    assert_eq!(Encoding::for_label(b"bogusbogusbogusbogus"), None);
}

//#[test]
pub fn test_decode_valid_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.decode(b"abc\x80\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{00E4}");
        }
    }
    assert_eq!(encoding, WINDOWS_1257);
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_invalid_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.decode(b"abc\x80\xA1\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert_eq!(encoding, WINDOWS_1257);
    assert!(had_errors);
}

//#[test]
pub fn test_decode_ascii_only_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.decode(b"abc");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "abc");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert_eq!(encoding, WINDOWS_1257);
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_as_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.decode(b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "\u{20AC}\u{00E4}");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert_eq!(encoding, UTF_8);
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_invalid_utf8_as_windows_1257_to_cow() {
    let (cow, encoding, had_errors) =
        WINDOWS_1257.decode(b"\xEF\xBB\xBF\xE2\x82\xAC\x80\xC3\xA4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert_eq!(encoding, UTF_8);
    assert!(had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_as_utf_8_to_cow() {
    let (cow, encoding, had_errors) = UTF_8.decode(b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "\u{20AC}\u{00E4}");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert_eq!(encoding, UTF_8);
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_invalid_utf8_as_utf_8_to_cow() {
    let (cow, encoding, had_errors) = UTF_8.decode(b"\xEF\xBB\xBF\xE2\x82\xAC\x80\xC3\xA4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert_eq!(encoding, UTF_8);
    assert!(had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_as_utf_8_to_cow_with_bom_removal() {
    let (cow, had_errors) = UTF_8.decode_with_bom_removal(b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "\u{20AC}\u{00E4}");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_as_windows_1257_to_cow_with_bom_removal() {
    let (cow, had_errors) =
        WINDOWS_1257.decode_with_bom_removal(b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(
                s,
                "\u{013C}\u{00BB}\u{00E6}\u{0101}\u{201A}\u{00AC}\u{0106}\u{00A4}"
            );
        }
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_valid_windows_1257_to_cow_with_bom_removal() {
    let (cow, had_errors) = WINDOWS_1257.decode_with_bom_removal(b"abc\x80\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{00E4}");
        }
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_invalid_windows_1257_to_cow_with_bom_removal() {
    let (cow, had_errors) = WINDOWS_1257.decode_with_bom_removal(b"abc\x80\xA1\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert!(had_errors);
}

//#[test]
pub fn test_decode_ascii_only_windows_1257_to_cow_with_bom_removal() {
    let (cow, had_errors) = WINDOWS_1257.decode_with_bom_removal(b"abc");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "abc");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_to_cow_without_bom_handling() {
    let (cow, had_errors) =
        UTF_8.decode_without_bom_handling(b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "\u{FEFF}\u{20AC}\u{00E4}");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_invalid_utf8_to_cow_without_bom_handling() {
    let (cow, had_errors) =
        UTF_8.decode_without_bom_handling(b"\xEF\xBB\xBF\xE2\x82\xAC\x80\xC3\xA4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "\u{FEFF}\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert!(had_errors);
}

//#[test]
pub fn test_decode_valid_windows_1257_to_cow_without_bom_handling() {
    let (cow, had_errors) = WINDOWS_1257.decode_without_bom_handling(b"abc\x80\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{00E4}");
        }
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_invalid_windows_1257_to_cow_without_bom_handling() {
    let (cow, had_errors) = WINDOWS_1257.decode_without_bom_handling(b"abc\x80\xA1\xE4");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, "abc\u{20AC}\u{FFFD}\u{00E4}");
        }
    }
    assert!(had_errors);
}

//#[test]
pub fn test_decode_ascii_only_windows_1257_to_cow_without_bom_handling() {
    let (cow, had_errors) = WINDOWS_1257.decode_without_bom_handling(b"abc");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, "abc");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert!(!had_errors);
}

//#[test]
pub fn test_decode_bomful_valid_utf8_to_cow_without_bom_handling_and_without_replacement() {
    match UTF_8.decode_without_bom_handling_and_without_replacement(
        b"\xEF\xBB\xBF\xE2\x82\xAC\xC3\xA4",
    ) {
        Some(cow) => match cow {
            Cow::Borrowed(s) => {
                assert_eq!(s, "\u{FEFF}\u{20AC}\u{00E4}");
            }
            Cow::Owned(_) => unreachable!(),
        },
        None => unreachable!(),
    }
}

//#[test]
pub fn test_decode_bomful_invalid_utf8_to_cow_without_bom_handling_and_without_replacement() {
    assert!(UTF_8
        .decode_without_bom_handling_and_without_replacement(
            b"\xEF\xBB\xBF\xE2\x82\xAC\x80\xC3\xA4"
        )
        .is_none());
}

//#[test]
pub fn test_decode_valid_windows_1257_to_cow_without_bom_handling_and_without_replacement() {
    match WINDOWS_1257.decode_without_bom_handling_and_without_replacement(b"abc\x80\xE4") {
        Some(cow) => match cow {
            Cow::Borrowed(_) => unreachable!(),
            Cow::Owned(s) => {
                assert_eq!(s, "abc\u{20AC}\u{00E4}");
            }
        },
        None => unreachable!(),
    }
}

//#[test]
pub fn test_decode_invalid_windows_1257_to_cow_without_bom_handling_and_without_replacement() {
    assert!(WINDOWS_1257
        .decode_without_bom_handling_and_without_replacement(b"abc\x80\xA1\xE4")
        .is_none());
}

//#[test]
pub fn test_decode_ascii_only_windows_1257_to_cow_without_bom_handling_and_without_replacement() {
    match WINDOWS_1257.decode_without_bom_handling_and_without_replacement(b"abc") {
        Some(cow) => match cow {
            Cow::Borrowed(s) => {
                assert_eq!(s, "abc");
            }
            Cow::Owned(_) => unreachable!(),
        },
        None => unreachable!(),
    }
}

//#[test]
pub fn test_encode_ascii_only_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.encode("abc");
    match cow {
        Cow::Borrowed(s) => {
            assert_eq!(s, b"abc");
        }
        Cow::Owned(_) => unreachable!(),
    }
    assert_eq!(encoding, WINDOWS_1257);
    assert!(!had_errors);
}

//#[test]
pub fn test_encode_valid_windows_1257_to_cow() {
    let (cow, encoding, had_errors) = WINDOWS_1257.encode("abc\u{20AC}\u{00E4}");
    match cow {
        Cow::Borrowed(_) => unreachable!(),
        Cow::Owned(s) => {
            assert_eq!(s, b"abc\x80\xE4");
        }
    }
    assert_eq!(encoding, WINDOWS_1257);
    assert!(!had_errors);
}

//#[test]
pub fn test_utf16_space_with_one_bom_byte() {
    let mut decoder = UTF_16LE.new_decoder();
    let mut dst = [0u16; 12];
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_utf8_space_with_one_bom_byte() {
    let mut decoder = UTF_8.new_decoder();
    let mut dst = [0u16; 12];
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_utf16_space_with_two_bom_bytes() {
    let mut decoder = UTF_16LE.new_decoder();
    let mut dst = [0u16; 12];
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xEF", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xBB", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_utf8_space_with_two_bom_bytes() {
    let mut decoder = UTF_8.new_decoder();
    let mut dst = [0u16; 12];
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xEF", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xBB", &mut dst[..needed], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let needed = decoder.max_utf16_buffer_length(1).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF", &mut dst[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_utf16_space_with_one_bom_byte_and_a_second_byte_in_same_call() {
    let mut decoder = UTF_16LE.new_decoder();
    let mut dst = [0u16; 12];
    {
        let needed = decoder.max_utf16_buffer_length(2).unwrap();
        let (result, _, _, _) = decoder.decode_to_utf16(b"\xFF\xFF", &mut dst[..needed], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_too_short_buffer_with_iso_2022_jp_ascii_from_utf8() {
    let mut dst = [0u8; 8];
    let mut encoder = ISO_2022_JP.new_encoder();
    {
        let (result, _, _, _) = encoder.encode_from_utf8("", &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf8("", &mut dst[..], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_too_short_buffer_with_iso_2022_jp_roman_from_utf8() {
    let mut dst = [0u8; 16];
    let mut encoder = ISO_2022_JP.new_encoder();
    {
        let (result, _, _, _) = encoder.encode_from_utf8("\u{A5}", &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf8("", &mut dst[..8], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf8("", &mut dst[..8], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
}

//#[test]
pub fn test_buffer_end_iso_2022_jp_from_utf8() {
    let mut dst = [0u8; 18];
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf8("\u{A5}\u{1F4A9}", &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) = encoder.encode_from_utf8("\u{A5}\u{1F4A9}", &mut dst[..], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) = encoder.encode_from_utf8("\u{1F4A9}", &mut dst[..13], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) = encoder.encode_from_utf8("\u{1F4A9}", &mut dst[..13], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_too_short_buffer_with_iso_2022_jp_ascii_from_utf16() {
    let mut dst = [0u8; 8];
    let mut encoder = ISO_2022_JP.new_encoder();
    {
        let (result, _, _, _) = encoder.encode_from_utf16(&[0u16; 0], &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf16(&[0u16; 0], &mut dst[..], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_too_short_buffer_with_iso_2022_jp_roman_from_utf16() {
    let mut dst = [0u8; 16];
    let mut encoder = ISO_2022_JP.new_encoder();
    {
        let (result, _, _, _) = encoder.encode_from_utf16(&[0xA5u16], &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf16(&[0u16; 0], &mut dst[..8], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let (result, _, _, _) = encoder.encode_from_utf16(&[0u16; 0], &mut dst[..8], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
}

//#[test]
pub fn test_buffer_end_iso_2022_jp_from_utf16() {
    let mut dst = [0u8; 18];
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf16(&[0xA5u16, 0xD83Du16, 0xDCA9u16], &mut dst[..], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf16(&[0xA5u16, 0xD83Du16, 0xDCA9u16], &mut dst[..], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf16(&[0xD83Du16, 0xDCA9u16], &mut dst[..13], false);
        assert_eq!(result, CoderResult::InputEmpty);
    }
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf16(&[0xD83Du16, 0xDCA9u16], &mut dst[..13], true);
        assert_eq!(result, CoderResult::InputEmpty);
    }
}

//#[test]
pub fn test_hash() {
    let mut encodings = ::std::collections::HashSet::new();
    encodings.insert(UTF_8);
    encodings.insert(ISO_2022_JP);
    assert!(encodings.contains(UTF_8));
    assert!(encodings.contains(ISO_2022_JP));
    assert!(!encodings.contains(WINDOWS_1252));
    encodings.remove(ISO_2022_JP);
    assert!(!encodings.contains(ISO_2022_JP));
}

//#[test]
pub fn test_iso_2022_jp_ncr_extra_from_utf16() {
    let mut dst = [0u8; 17];
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf16(&[0x3041u16, 0xFFFFu16], &mut dst[..], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
}

//#[test]
pub fn test_iso_2022_jp_ncr_extra_from_utf8() {
    let mut dst = [0u8; 17];
    {
        let mut encoder = ISO_2022_JP.new_encoder();
        let (result, _, _, _) =
            encoder.encode_from_utf8("\u{3041}\u{FFFF}", &mut dst[..], true);
        assert_eq!(result, CoderResult::OutputFull);
    }
}

//#[test]
pub fn test_max_length_with_bom_to_utf8() {
    let mut output = [0u8; 20];
    let mut decoder = REPLACEMENT.new_decoder();
    let input = b"\xEF\xBB\xBFA";
    {
        let needed = decoder
            .max_utf8_buffer_length_without_replacement(input.len())
            .unwrap();
        let (result, read, written) =
            decoder.decode_to_utf8_without_replacement(input, &mut output[..needed], true);
        assert_eq!(result, DecoderResult::InputEmpty);
        assert_eq!(read, input.len());
        assert_eq!(written, 1);
        assert_eq!(output[0], 0x41);
    }
}

// XXX:TODO: serde
//#[cfg(feature = "serde")]
//#[test]
//pub fn test_serde() {
//    let demo = Demo {
//        num: 42,
//        name: "foo".into(),
//        enc: UTF_8,
//    };
//
//    let serialized = serde_json::to_string(&demo).unwrap();
//
//    let deserialized: Demo = serde_json::from_str(&serialized).unwrap();
//    assert_eq!(deserialized, demo);
//
//    let bincoded = bincode::serialize(&demo).unwrap();
//    let debincoded: Demo = bincode::deserialize(&bincoded[..]).unwrap();
//    assert_eq!(debincoded, demo);
//}

//#[test]
pub fn test_is_single_byte() {
    assert!(!BIG5.is_single_byte());
    assert!(!EUC_JP.is_single_byte());
    assert!(!EUC_KR.is_single_byte());
    assert!(!GB18030.is_single_byte());
    assert!(!GBK.is_single_byte());
    assert!(!REPLACEMENT.is_single_byte());
    assert!(!SHIFT_JIS.is_single_byte());
    assert!(!UTF_8.is_single_byte());
    assert!(!UTF_16BE.is_single_byte());
    assert!(!UTF_16LE.is_single_byte());
    assert!(!ISO_2022_JP.is_single_byte());

    assert!(IBM866.is_single_byte());
    assert!(ISO_8859_2.is_single_byte());
    assert!(ISO_8859_3.is_single_byte());
    assert!(ISO_8859_4.is_single_byte());
    assert!(ISO_8859_5.is_single_byte());
    assert!(ISO_8859_6.is_single_byte());
    assert!(ISO_8859_7.is_single_byte());
    assert!(ISO_8859_8.is_single_byte());
    assert!(ISO_8859_10.is_single_byte());
    assert!(ISO_8859_13.is_single_byte());
    assert!(ISO_8859_14.is_single_byte());
    assert!(ISO_8859_15.is_single_byte());
    assert!(ISO_8859_16.is_single_byte());
    assert!(ISO_8859_8_I.is_single_byte());
    assert!(KOI8_R.is_single_byte());
    assert!(KOI8_U.is_single_byte());
    assert!(MACINTOSH.is_single_byte());
    assert!(WINDOWS_874.is_single_byte());
    assert!(WINDOWS_1250.is_single_byte());
    assert!(WINDOWS_1251.is_single_byte());
    assert!(WINDOWS_1252.is_single_byte());
    assert!(WINDOWS_1253.is_single_byte());
    assert!(WINDOWS_1254.is_single_byte());
    assert!(WINDOWS_1255.is_single_byte());
    assert!(WINDOWS_1256.is_single_byte());
    assert!(WINDOWS_1257.is_single_byte());
    assert!(WINDOWS_1258.is_single_byte());
    assert!(X_MAC_CYRILLIC.is_single_byte());
    assert!(X_USER_DEFINED.is_single_byte());
}
