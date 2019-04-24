//use super::*;
use std::borrow::*;
use std::prelude::v1::*;
use encoding_rs::mem::*;

//#[test]
pub fn test_is_ascii_success() {
    let mut src: Vec<u8> = Vec::with_capacity(128);
    src.resize(128, 0);
    for i in 0..src.len() {
        src[i] = i as u8;
    }
    for i in 0..src.len() {
        assert!(is_ascii(&src[i..]));
    }
}

//#[test]
pub fn test_is_ascii_fail() {
    let mut src: Vec<u8> = Vec::with_capacity(128);
    src.resize(128, 0);
    for i in 0..src.len() {
        src[i] = i as u8;
    }
    for i in 0..src.len() {
        let tail = &mut src[i..];
        for j in 0..tail.len() {
            tail[j] = 0xA0;
            assert!(!is_ascii(tail));
        }
    }
}

//#[test]
pub fn test_is_basic_latin_success() {
    let mut src: Vec<u16> = Vec::with_capacity(128);
    src.resize(128, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        assert!(is_basic_latin(&src[i..]));
    }
}

//#[test]
pub fn test_is_basic_latin_fail() {
    let mut src: Vec<u16> = Vec::with_capacity(128);
    src.resize(128, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let tail = &mut src[i..];
        for j in 0..tail.len() {
            tail[j] = 0xA0;
            assert!(!is_basic_latin(tail));
        }
    }
}

//#[test]
pub fn test_is_utf16_latin1_success() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        assert!(is_utf16_latin1(&src[i..]));
        assert_eq!(
            check_utf16_for_latin1_and_bidi(&src[i..]),
            Latin1Bidi::Latin1
        );
    }
}

//#[test]
pub fn test_is_utf16_latin1_fail() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let tail = &mut src[i..];
        for j in 0..tail.len() {
            tail[j] = 0x100 + j as u16;
            assert!(!is_utf16_latin1(tail));
            assert_ne!(check_utf16_for_latin1_and_bidi(tail), Latin1Bidi::Latin1);
        }
    }
}

//#[test]
pub fn test_is_str_latin1_success() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let s = String::from_utf16(&src[i..]).unwrap();
        assert!(is_str_latin1(&s[..]));
        assert_eq!(check_str_for_latin1_and_bidi(&s[..]), Latin1Bidi::Latin1);
    }
}

//#[test]
pub fn test_is_str_latin1_fail() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let tail = &mut src[i..];
        for j in 0..tail.len() {
            tail[j] = 0x100 + j as u16;
            let s = String::from_utf16(tail).unwrap();
            assert!(!is_str_latin1(&s[..]));
            assert_ne!(check_str_for_latin1_and_bidi(&s[..]), Latin1Bidi::Latin1);
        }
    }
}

//#[test]
pub fn test_is_utf8_latin1_success() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let s = String::from_utf16(&src[i..]).unwrap();
        assert!(is_utf8_latin1(s.as_bytes()));
        assert_eq!(
            check_utf8_for_latin1_and_bidi(s.as_bytes()),
            Latin1Bidi::Latin1
        );
    }
}

//#[test]
pub fn test_is_utf8_latin1_fail() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    for i in 0..src.len() {
        src[i] = i as u16;
    }
    for i in 0..src.len() {
        let tail = &mut src[i..];
        for j in 0..tail.len() {
            tail[j] = 0x100 + j as u16;
            let s = String::from_utf16(tail).unwrap();
            assert!(!is_utf8_latin1(s.as_bytes()));
            assert_ne!(
                check_utf8_for_latin1_and_bidi(s.as_bytes()),
                Latin1Bidi::Latin1
            );
        }
    }
}

//#[test]
pub fn test_is_utf8_latin1_invalid() {
    assert!(!is_utf8_latin1(b"\xC3"));
    assert!(!is_utf8_latin1(b"a\xC3"));
    assert!(!is_utf8_latin1(b"\xFF"));
    assert!(!is_utf8_latin1(b"a\xFF"));
    assert!(!is_utf8_latin1(b"\xC3\xFF"));
    assert!(!is_utf8_latin1(b"a\xC3\xFF"));
}

//#[test]
pub fn test_convert_utf8_to_utf16() {
    let src = "abcdefghijklmnopqrstu\u{1F4A9}v\u{2603}w\u{00B6}xyzz";
    let mut dst: Vec<u16> = Vec::with_capacity(src.len() + 1);
    dst.resize(src.len() + 1, 0);
    let len = convert_utf8_to_utf16(src.as_bytes(), &mut dst[..]);
    dst.truncate(len);
    let reference: Vec<u16> = src.encode_utf16().collect();
    assert_eq!(dst, reference);
}

//#[test]
pub fn test_convert_str_to_utf16() {
    let src = "abcdefghijklmnopqrstu\u{1F4A9}v\u{2603}w\u{00B6}xyzz";
    let mut dst: Vec<u16> = Vec::with_capacity(src.len());
    dst.resize(src.len(), 0);
    let len = convert_str_to_utf16(src, &mut dst[..]);
    dst.truncate(len);
    let reference: Vec<u16> = src.encode_utf16().collect();
    assert_eq!(dst, reference);
}

//#[test]
pub fn test_convert_utf16_to_utf8_partial() {
    let reference = "abcdefghijklmnopqrstu\u{1F4A9}v\u{2603}w\u{00B6}xyzz";
    let src: Vec<u16> = reference.encode_utf16().collect();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len() * 3 + 1);
    dst.resize(src.len() * 3 + 1, 0);
    let (read, written) = convert_utf16_to_utf8_partial(&src[..], &mut dst[..24]);
    let len = written + convert_utf16_to_utf8(&src[read..], &mut dst[written..]);
    dst.truncate(len);
    assert_eq!(dst, reference.as_bytes());
}

//#[test]
pub fn test_convert_utf16_to_utf8() {
    let reference = "abcdefghijklmnopqrstu\u{1F4A9}v\u{2603}w\u{00B6}xyzz";
    let src: Vec<u16> = reference.encode_utf16().collect();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len() * 3 + 1);
    dst.resize(src.len() * 3 + 1, 0);
    let len = convert_utf16_to_utf8(&src[..], &mut dst[..]);
    dst.truncate(len);
    assert_eq!(dst, reference.as_bytes());
}

//#[test]
pub fn test_convert_latin1_to_utf16() {
    let mut src: Vec<u8> = Vec::with_capacity(256);
    src.resize(256, 0);
    let mut reference: Vec<u16> = Vec::with_capacity(256);
    reference.resize(256, 0);
    for i in 0..256 {
        src[i] = i as u8;
        reference[i] = i as u16;
    }
    let mut dst: Vec<u16> = Vec::with_capacity(src.len());
    dst.resize(src.len(), 0);
    convert_latin1_to_utf16(&src[..], &mut dst[..]);
    assert_eq!(dst, reference);
}

//#[test]
pub fn test_convert_latin1_to_utf8_partial() {
    let mut dst = [0u8, 2];
    let (read, written) = convert_latin1_to_utf8_partial(b"a\xFF", &mut dst[..]);
    assert_eq!(read, 1);
    assert_eq!(written, 1);
}

//#[test]
pub fn test_convert_latin1_to_utf8() {
    let mut src: Vec<u8> = Vec::with_capacity(256);
    src.resize(256, 0);
    let mut reference: Vec<u16> = Vec::with_capacity(256);
    reference.resize(256, 0);
    for i in 0..256 {
        src[i] = i as u8;
        reference[i] = i as u16;
    }
    let s = String::from_utf16(&reference[..]).unwrap();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len() * 2);
    dst.resize(src.len() * 2, 0);
    let len = convert_latin1_to_utf8(&src[..], &mut dst[..]);
    dst.truncate(len);
    assert_eq!(&dst[..], s.as_bytes());
}

//#[test]
pub fn test_convert_utf8_to_latin1_lossy() {
    let mut reference: Vec<u8> = Vec::with_capacity(256);
    reference.resize(256, 0);
    let mut src16: Vec<u16> = Vec::with_capacity(256);
    src16.resize(256, 0);
    for i in 0..256 {
        src16[i] = i as u16;
        reference[i] = i as u8;
    }
    let src = String::from_utf16(&src16[..]).unwrap();
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    dst.resize(src.len(), 0);
    let len = convert_utf8_to_latin1_lossy(src.as_bytes(), &mut dst[..]);
    dst.truncate(len);
    assert_eq!(dst, reference);
}

#[cfg(all(debug_assertions, not(fuzzing)))]
#[test]
#[should_panic]
fn test_convert_utf8_to_latin1_lossy_panics() {
    let mut dst = [0u8; 16];
    let _ = convert_utf8_to_latin1_lossy("\u{100}".as_bytes(), &mut dst[..]);
}

//#[test]
pub fn test_convert_utf16_to_latin1_lossy() {
    let mut src: Vec<u16> = Vec::with_capacity(256);
    src.resize(256, 0);
    let mut reference: Vec<u8> = Vec::with_capacity(256);
    reference.resize(256, 0);
    for i in 0..256 {
        src[i] = i as u16;
        reference[i] = i as u8;
    }
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    dst.resize(src.len(), 0);
    convert_utf16_to_latin1_lossy(&src[..], &mut dst[..]);
    assert_eq!(dst, reference);
}

#[test]
// #[should_panic]
fn test_convert_utf16_to_latin1_lossy_panics() {
    let mut dst = [0u8; 16];
    let _ = convert_utf16_to_latin1_lossy(&[0x0100u16], &mut dst[..]);
}

//#[test]
pub fn test_utf16_valid_up_to() {
    let valid = vec![
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0x2603u16,
        0xD83Du16, 0xDCA9u16, 0x00B6u16,
    ];
    assert_eq!(utf16_valid_up_to(&valid[..]), 16);
    let lone_high = vec![
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0x2603u16, 0xD83Du16, 0x00B6u16,
    ];
    assert_eq!(utf16_valid_up_to(&lone_high[..]), 14);
    let lone_low = vec![
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0x2603u16, 0xDCA9u16, 0x00B6u16,
    ];
    assert_eq!(utf16_valid_up_to(&lone_low[..]), 14);
    let lone_high_at_end = vec![
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0x2603u16, 0x00B6u16, 0xD83Du16,
    ];
    assert_eq!(utf16_valid_up_to(&lone_high_at_end[..]), 15);
}

//#[test]
pub fn test_ensure_utf16_validity() {
    let mut src = vec![
        0u16, 0xD83Du16, 0u16, 0u16, 0u16, 0xD83Du16, 0xDCA9u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0xDCA9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
    ];
    let reference = vec![
        0u16, 0xFFFDu16, 0u16, 0u16, 0u16, 0xD83Du16, 0xDCA9u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0xFFFDu16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
    ];
    ensure_utf16_validity(&mut src[..]);
    assert_eq!(src, reference);
}

//#[test]
pub fn test_is_char_bidi() {
    assert!(!is_char_bidi('a'));
    assert!(!is_char_bidi('\u{03B1}'));
    assert!(!is_char_bidi('\u{3041}'));
    assert!(!is_char_bidi('\u{1F4A9}'));
    assert!(!is_char_bidi('\u{FE00}'));
    assert!(!is_char_bidi('\u{202C}'));
    assert!(!is_char_bidi('\u{FEFF}'));
    assert!(is_char_bidi('\u{0590}'));
    assert!(is_char_bidi('\u{08FF}'));
    assert!(is_char_bidi('\u{061C}'));
    assert!(is_char_bidi('\u{FB50}'));
    assert!(is_char_bidi('\u{FDFF}'));
    assert!(is_char_bidi('\u{FE70}'));
    assert!(is_char_bidi('\u{FEFE}'));
    assert!(is_char_bidi('\u{200F}'));
    assert!(is_char_bidi('\u{202B}'));
    assert!(is_char_bidi('\u{202E}'));
    assert!(is_char_bidi('\u{2067}'));
    assert!(is_char_bidi('\u{10800}'));
    assert!(is_char_bidi('\u{10FFF}'));
    assert!(is_char_bidi('\u{1E800}'));
    assert!(is_char_bidi('\u{1EFFF}'));
}

//#[test]
pub fn test_is_utf16_code_unit_bidi() {
    assert!(!is_utf16_code_unit_bidi(0x0062));
    assert!(!is_utf16_code_unit_bidi(0x03B1));
    assert!(!is_utf16_code_unit_bidi(0x3041));
    assert!(!is_utf16_code_unit_bidi(0xD801));
    assert!(!is_utf16_code_unit_bidi(0xFE00));
    assert!(!is_utf16_code_unit_bidi(0x202C));
    assert!(!is_utf16_code_unit_bidi(0xFEFF));
    assert!(is_utf16_code_unit_bidi(0x0590));
    assert!(is_utf16_code_unit_bidi(0x08FF));
    assert!(is_utf16_code_unit_bidi(0x061C));
    assert!(is_utf16_code_unit_bidi(0xFB1D));
    assert!(is_utf16_code_unit_bidi(0xFB50));
    assert!(is_utf16_code_unit_bidi(0xFDFF));
    assert!(is_utf16_code_unit_bidi(0xFE70));
    assert!(is_utf16_code_unit_bidi(0xFEFE));
    assert!(is_utf16_code_unit_bidi(0x200F));
    assert!(is_utf16_code_unit_bidi(0x202B));
    assert!(is_utf16_code_unit_bidi(0x202E));
    assert!(is_utf16_code_unit_bidi(0x2067));
    assert!(is_utf16_code_unit_bidi(0xD802));
    assert!(is_utf16_code_unit_bidi(0xD803));
    assert!(is_utf16_code_unit_bidi(0xD83A));
    assert!(is_utf16_code_unit_bidi(0xD83B));
}

//#[test]
pub fn test_is_str_bidi() {
    assert!(!is_str_bidi("abcdefghijklmnopaabcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{03B1}abcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{3041}abcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{1F4A9}abcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{FE00}abcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{202C}abcdefghijklmnop"));
    assert!(!is_str_bidi("abcdefghijklmnop\u{FEFF}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{0590}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{08FF}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{061C}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{FB50}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{FDFF}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{FE70}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{FEFE}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{200F}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{202B}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{202E}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{2067}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{10800}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{10FFF}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{1E800}abcdefghijklmnop"));
    assert!(is_str_bidi("abcdefghijklmnop\u{1EFFF}abcdefghijklmnop"));
}

//#[test]
pub fn test_is_utf8_bidi() {
    assert!(!is_utf8_bidi(
        "abcdefghijklmnopaabcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{03B1}abcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{3041}abcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{1F4A9}abcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{FE00}abcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{202C}abcdefghijklmnop".as_bytes()
    ));
    assert!(!is_utf8_bidi(
        "abcdefghijklmnop\u{FEFF}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{0590}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{08FF}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{061C}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{FB50}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{FDFF}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{FE70}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{FEFE}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{200F}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{202B}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{202E}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{2067}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{10800}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{10FFF}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{1E800}abcdefghijklmnop".as_bytes()
    ));
    assert!(is_utf8_bidi(
        "abcdefghijklmnop\u{1EFFF}abcdefghijklmnop".as_bytes()
    ));
}

//#[test]
pub fn test_is_utf16_bidi() {
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0062, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x03B1, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x3041, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD801, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFE00, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202C, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(!is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFEFF, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0590, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x08FF, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x061C, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFB1D, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFB50, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFDFF, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFE70, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFEFE, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x200F, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202B, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202E, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x2067, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD802, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD803, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD83A, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));
    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD83B, 0x62, 0x63, 0x64, 0x65, 0x66,
        0x67, 0x68, 0x69,
    ]));

    assert!(is_utf16_bidi(&[
        0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0590, 0x3041, 0x62, 0x63, 0x64, 0x65,
        0x66, 0x67, 0x68, 0x69,
    ]));
}

//#[test]
pub fn test_check_str_for_latin1_and_bidi() {
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnopaabcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{03B1}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{3041}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{1F4A9}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FE00}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{202C}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FEFF}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{0590}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{08FF}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{061C}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FB50}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FDFF}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FE70}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{FEFE}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{200F}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{202B}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{202E}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{2067}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{10800}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{10FFF}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{1E800}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_str_for_latin1_and_bidi("abcdefghijklmnop\u{1EFFF}abcdefghijklmnop"),
        Latin1Bidi::Bidi
    );
}

//#[test]
pub fn test_check_utf8_for_latin1_and_bidi() {
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnopaabcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{03B1}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{3041}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{1F4A9}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FE00}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{202C}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FEFF}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{0590}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{08FF}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{061C}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FB50}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FDFF}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FE70}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{FEFE}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{200F}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{202B}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{202E}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{2067}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{10800}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{10FFF}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{1E800}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf8_for_latin1_and_bidi("abcdefghijklmnop\u{1EFFF}abcdefghijklmnop".as_bytes()),
        Latin1Bidi::Bidi
    );
}

//#[test]
pub fn test_check_utf16_for_latin1_and_bidi() {
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0062, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x03B1, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x3041, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD801, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFE00, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202C, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_ne!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFEFF, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0590, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x08FF, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x061C, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFB1D, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFB50, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFDFF, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFE70, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xFEFE, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x200F, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202B, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x202E, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x2067, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD802, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD803, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD83A, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0xD83B, 0x62, 0x63, 0x64, 0x65,
            0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );

    assert_eq!(
        check_utf16_for_latin1_and_bidi(&[
            0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x0590, 0x3041, 0x62, 0x63, 0x64,
            0x65, 0x66, 0x67, 0x68, 0x69,
        ]),
        Latin1Bidi::Bidi
    );
}

#[inline(always)]
pub fn reference_is_char_bidi(c: char) -> bool {
    match c {
        '\u{0590}'..='\u{08FF}'
        | '\u{FB1D}'..='\u{FDFF}'
        | '\u{FE70}'..='\u{FEFE}'
        | '\u{10800}'..='\u{10FFF}'
        | '\u{1E800}'..='\u{1EFFF}'
        | '\u{200F}'
        | '\u{202B}'
        | '\u{202E}'
        | '\u{2067}' => true,
        _ => false,
    }
}

#[inline(always)]
pub fn reference_is_utf16_code_unit_bidi(u: u16) -> bool {
    match u {
        0x0590..=0x08FF
        | 0xFB1D..=0xFDFF
        | 0xFE70..=0xFEFE
        | 0xD802
        | 0xD803
        | 0xD83A
        | 0xD83B
        | 0x200F
        | 0x202B
        | 0x202E
        | 0x2067 => true,
        _ => false,
    }
}

//#[test]
pub fn test_is_char_bidi_thoroughly() {
    for i in 0..0xD800u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        assert_eq!(is_char_bidi(c), reference_is_char_bidi(c));
    }
    for i in 0xE000..0x110000u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        assert_eq!(is_char_bidi(c), reference_is_char_bidi(c));
    }
}

//#[test]
pub fn test_is_utf16_code_unit_bidi_thoroughly() {
    for i in 0..0x10000u32 {
        let u = i as u16;
        assert_eq!(
            is_utf16_code_unit_bidi(u),
            reference_is_utf16_code_unit_bidi(u)
        );
    }
}

//#[test]
pub fn test_is_str_bidi_thoroughly() {
    let mut buf = [0; 4];
    for i in 0..0xD800u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        assert_eq!(
            is_str_bidi(c.encode_utf8(&mut buf[..])),
            reference_is_char_bidi(c)
        );
    }
    for i in 0xE000..0x110000u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        assert_eq!(
            is_str_bidi(c.encode_utf8(&mut buf[..])),
            reference_is_char_bidi(c)
        );
    }
}

//#[test]
pub fn test_is_utf8_bidi_thoroughly() {
    let mut buf = [0; 8];
    for i in 0..0xD800u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        let expect = reference_is_char_bidi(c);
        {
            let len = {
                let bytes = c.encode_utf8(&mut buf[..]).as_bytes();
                assert_eq!(is_utf8_bidi(bytes), expect);
                bytes.len()
            };
            {
                let tail = &mut buf[len..];
                for b in tail.iter_mut() {
                    *b = 0;
                }
            }
        }
        assert_eq!(is_utf8_bidi(&buf[..]), expect);
    }
    for i in 0xE000..0x110000u32 {
        let c: char = ::std::char::from_u32(i).unwrap();
        let expect = reference_is_char_bidi(c);
        {
            let len = {
                let bytes = c.encode_utf8(&mut buf[..]).as_bytes();
                assert_eq!(is_utf8_bidi(bytes), expect);
                bytes.len()
            };
            {
                let tail = &mut buf[len..];
                for b in tail.iter_mut() {
                    *b = 0;
                }
            }
        }
        assert_eq!(is_utf8_bidi(&buf[..]), expect);
    }
}

//#[test]
pub fn test_is_utf16_bidi_thoroughly() {
    let mut buf = [0; 32];
    for i in 0..0x10000u32 {
        let u = i as u16;
        buf[15] = u;
        assert_eq!(
            is_utf16_bidi(&buf[..]),
            reference_is_utf16_code_unit_bidi(u)
        );
    }
}

//#[test]
pub fn test_is_utf8_bidi_edge_cases() {
    assert!(!is_utf8_bidi(b"\xD5\xBF\x61"));
    assert!(!is_utf8_bidi(b"\xD6\x80\x61"));
    assert!(!is_utf8_bidi(b"abc"));
    assert!(is_utf8_bidi(b"\xD5\xBF\xC2"));
    assert!(is_utf8_bidi(b"\xD6\x80\xC2"));
    assert!(is_utf8_bidi(b"ab\xC2"));
}

//#[test]
pub fn test_decode_latin1() {
    match decode_latin1(b"ab") {
        Cow::Borrowed(s) => {
            assert_eq!(s, "ab");
        }
        Cow::Owned(_) => {
            unreachable!("Should have borrowed");
        }
    }
    assert_eq!(decode_latin1(b"a\xE4"), "a\u{E4}");
}

//#[test]
pub fn test_encode_latin1_lossy() {
    match encode_latin1_lossy("ab") {
        Cow::Borrowed(s) => {
            assert_eq!(s, b"ab");
        }
        Cow::Owned(_) => {
            unreachable!("Should have borrowed");
        }
    }
    assert_eq!(encode_latin1_lossy("a\u{E4}"), &(b"a\xE4")[..]);
}
