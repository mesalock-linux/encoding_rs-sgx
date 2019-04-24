use encoding_rs::simd_funcs::*;
use std::prelude::v1::*;

//#[test]
pub fn test_unpack() {
    let ascii: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let basic_latin: [u16; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let simd = unsafe { load16_unaligned(ascii.as_ptr()) };
    let mut vec = Vec::with_capacity(16);
    vec.resize(16, 0u16);
    let (first, second) = simd_unpack(simd);
    let ptr = vec.as_mut_ptr();
    unsafe {
        store8_unaligned(ptr, first);
        store8_unaligned(ptr.add(8), second);
    }
    assert_eq!(&vec[..], &basic_latin[..]);
}

//#[test]
pub fn test_simd_is_basic_latin_success() {
    let ascii: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let basic_latin: [u16; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let first = unsafe { load8_unaligned(basic_latin.as_ptr()) };
    let second = unsafe { load8_unaligned(basic_latin.as_ptr().add(8)) };
    let mut vec = Vec::with_capacity(16);
    vec.resize(16, 0u8);
    let ptr = vec.as_mut_ptr();
    assert!(simd_is_basic_latin(first | second));
    unsafe {
        store16_unaligned(ptr, simd_pack(first, second));
    }
    assert_eq!(&vec[..], &ascii[..]);
}

//#[test]
pub fn test_simd_is_basic_latin_c0() {
    let input: [u16; 16] = [
        0x61, 0x62, 0x63, 0x81, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let first = unsafe { load8_unaligned(input.as_ptr()) };
    let second = unsafe { load8_unaligned(input.as_ptr().add(8)) };
    assert!(!simd_is_basic_latin(first | second));
}

//#[test]
pub fn test_simd_is_basic_latin_0fff() {
    let input: [u16; 16] = [
        0x61, 0x62, 0x63, 0x0FFF, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let first = unsafe { load8_unaligned(input.as_ptr()) };
    let second = unsafe { load8_unaligned(input.as_ptr().add(8)) };
    assert!(!simd_is_basic_latin(first | second));
}

//#[test]
pub fn test_simd_is_basic_latin_ffff() {
    let input: [u16; 16] = [
        0x61, 0x62, 0x63, 0xFFFF, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let first = unsafe { load8_unaligned(input.as_ptr()) };
    let second = unsafe { load8_unaligned(input.as_ptr().add(8)) };
    assert!(!simd_is_basic_latin(first | second));
}

//#[test]
pub fn test_simd_is_ascii_success() {
    let ascii: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let simd = unsafe { load16_unaligned(ascii.as_ptr()) };
    assert!(simd_is_ascii(simd));
}

//#[test]
pub fn test_simd_is_ascii_failure() {
    let input: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let simd = unsafe { load16_unaligned(input.as_ptr()) };
    assert!(!simd_is_ascii(simd));
}

#[cfg(target_feature = "sse2")]
//#[test]
pub fn test_check_ascii() {
    let input: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let simd = unsafe { load16_unaligned(input.as_ptr()) };
    let mask = mask_ascii(simd);
    assert_ne!(mask, 0);
    assert_eq!(mask.trailing_zeros(), 4);
}

//#[test]
pub fn test_alu() {
    let input: [u8; 16] = [
        0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71, 0x72, 0x73, 0x74,
        0x75, 0x76,
    ];
    let mut alu = 0u64;
    unsafe {
        ::std::ptr::copy_nonoverlapping(input.as_ptr(), &mut alu as *mut u64 as *mut u8, 8);
    }
    let masked = alu & 0x8080808080808080;
    assert_eq!(masked.trailing_zeros(), 39);
}
