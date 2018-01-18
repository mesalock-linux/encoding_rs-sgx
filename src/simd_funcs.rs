// Copyright 2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd::u8x16;
use simd::u16x8;
use simd::Simd;

// TODO: Migrate unaligned access to stdlib code if/when the RFC
// https://github.com/rust-lang/rfcs/pull/1725 is implemented.

#[inline(always)]
pub unsafe fn load16_unaligned(ptr: *const u8) -> u8x16 {
    let mut simd = ::std::mem::uninitialized();
    ::std::ptr::copy_nonoverlapping(ptr, &mut simd as *mut u8x16 as *mut u8, 16);
    simd
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn load16_aligned(ptr: *const u8) -> u8x16 {
    *(ptr as *const u8x16)
}

#[inline(always)]
pub unsafe fn store16_unaligned(ptr: *mut u8, s: u8x16) {
    ::std::ptr::copy_nonoverlapping(&s as *const u8x16 as *const u8, ptr, 16);
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn store16_aligned(ptr: *mut u8, s: u8x16) {
    *(ptr as *mut u8x16) = s;
}

#[inline(always)]
pub unsafe fn load8_unaligned(ptr: *const u16) -> u16x8 {
    let mut simd = ::std::mem::uninitialized();
    ::std::ptr::copy_nonoverlapping(ptr as *const u8, &mut simd as *mut u16x8 as *mut u8, 16);
    simd
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn load8_aligned(ptr: *const u16) -> u16x8 {
    *(ptr as *const u16x8)
}

#[inline(always)]
pub unsafe fn store8_unaligned(ptr: *mut u16, s: u16x8) {
    ::std::ptr::copy_nonoverlapping(&s as *const u16x8 as *const u8, ptr as *mut u8, 16);
}

#[allow(dead_code)]
#[inline(always)]
pub unsafe fn store8_aligned(ptr: *mut u16, s: u16x8) {
    *(ptr as *mut u16x8) = s;
}

extern "platform-intrinsic" {
    fn simd_shuffle16<T: Simd, U: Simd<Elem = T::Elem>>(x: T, y: T, idx: [u32; 16]) -> U;
}

cfg_if! {
    if #[cfg(target_feature = "sse2")] {

        use simd::i16x8;
        use simd::i8x16;
        extern "platform-intrinsic" {
            fn x86_mm_movemask_epi8(x: i8x16) -> i32;
        }

        // Expose low-level mask instead of higher-level conclusion,
        // because the non-ASCII case would perform less well otherwise.
        #[inline(always)]
        pub fn mask_ascii(s: u8x16) -> i32 {
            unsafe {
                let signed: i8x16 = ::std::mem::transmute_copy(&s);
                x86_mm_movemask_epi8(signed)
            }
        }

    } else {

    }
}

cfg_if! {
    if #[cfg(target_feature = "sse2")] {
        #[inline(always)]
        pub fn simd_is_ascii(s: u8x16) -> bool {
            unsafe {
                let signed: i8x16 = ::std::mem::transmute_copy(&s);
                x86_mm_movemask_epi8(signed) == 0
            }
        }
    } else if #[cfg(target_arch = "aarch64")]{
        extern "platform-intrinsic" {
            fn aarch64_vmaxvq_u8(x: u8x16) -> u8;
        }

        #[inline(always)]
        pub fn simd_is_ascii(s: u8x16) -> bool {
            unsafe {
                aarch64_vmaxvq_u8(s) < 0x80
            }
        }
    } else {
        #[inline(always)]
        pub fn simd_is_ascii(s: u8x16) -> bool {
            let above_ascii = u8x16::splat(0x80);
            s.lt(above_ascii).all()
        }
    }
}

cfg_if! {
    if #[cfg(target_feature = "sse2")] {
        #[inline(always)]
        pub fn simd_is_str_latin1(s: u8x16) -> bool {
            if simd_is_ascii(s) {
                return true;
            }
            let above_str_latin1 = u8x16::splat(0xC4);
            s.lt(above_str_latin1).all()
        }
    } else if #[cfg(target_arch = "aarch64")]{
        #[inline(always)]
        pub fn simd_is_str_latin1(s: u8x16) -> bool {
            unsafe {
                aarch64_vmaxvq_u8(s) < 0xC4
            }
        }
    } else {
        #[inline(always)]
        pub fn simd_is_str_latin1(s: u8x16) -> bool {
            let above_str_latin1 = u8x16::splat(0xC4);
            s.lt(above_str_latin1).all()
        }
    }
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")]{
        extern "platform-intrinsic" {
            fn aarch64_vmaxvq_u16(x: u16x8) -> u16;
        }

        #[inline(always)]
        pub fn simd_is_basic_latin(s: u16x8) -> bool {
            unsafe {
                aarch64_vmaxvq_u16(s) < 0x80
            }
        }

        #[inline(always)]
        pub fn simd_is_latin1(s: u16x8) -> bool {
            unsafe {
                aarch64_vmaxvq_u16(s) < 0x100
            }
        }
    } else {
        #[inline(always)]
        pub fn simd_is_basic_latin(s: u16x8) -> bool {
            let above_ascii = u16x8::splat(0x80);
            s.lt(above_ascii).all()
        }

        #[inline(always)]
        pub fn simd_is_latin1(s: u16x8) -> bool {
            // For some reason, on SSE2 this formulation
            // seems faster in this case while the above
            // function is better the other way round...
            let highest_latin1 = u16x8::splat(0xFF);
            !s.gt(highest_latin1).any()
        }
    }
}

#[inline(always)]
pub fn contains_surrogates(s: u16x8) -> bool {
    let mask = u16x8::splat(0xF800);
    let surrogate_bits = u16x8::splat(0xD800);
    (s & mask).eq(surrogate_bits).any()
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")]{
        macro_rules! aarch64_return_false_if_below_hebrew {
            ($s:ident) => ({
                unsafe {
                    if aarch64_vmaxvq_u16($s) < 0x0590 {
                        return false;
                    }
                }
            })
        }

        macro_rules! non_aarch64_return_false_if_all {
            ($s:ident) => ()
        }
    } else {
        macro_rules! aarch64_return_false_if_below_hebrew {
            ($s:ident) => ()
        }

        macro_rules! non_aarch64_return_false_if_all {
            ($s:ident) => ({
                if $s.all() {
                    return false;
                }
            })
        }
    }
}

macro_rules! in_range16x8 {
    ($s:ident, $start:expr, $end:expr) => ({
        // SIMD sub is wrapping
        ($s - u16x8::splat($start)).lt(u16x8::splat($end - $start))
    })
}

#[inline(always)]
pub fn is_u16x8_bidi(s: u16x8) -> bool {
    // We try to first quickly refute the RTLness of the vector. If that
    // fails, we do the real RTL check, so in that case we end up wasting
    // the work for the up-front quick checks. Even the quick-check is
    // two-fold in order to return `false` ASAP if everything is below
    // Hebrew.

    aarch64_return_false_if_below_hebrew!(s);

    let below_hebrew = s.lt(u16x8::splat(0x0590));

    non_aarch64_return_false_if_all!(below_hebrew);

    if (below_hebrew | in_range16x8!(s, 0x0900, 0x200F) | in_range16x8!(s, 0x2068, 0xD802)).all() {
        return false;
    }

    // Quick refutation failed. Let's do the full check.

    (in_range16x8!(s, 0x0590, 0x0900) | in_range16x8!(s, 0xFB50, 0xFE00) | in_range16x8!(s, 0xFE70, 0xFF00) | in_range16x8!(s, 0xD802, 0xD804) | in_range16x8!(s, 0xD83A, 0xD83C) | s.eq(u16x8::splat(0x200F)) | s.eq(u16x8::splat(0x202B)) | s.eq(u16x8::splat(0x202E)) | s.eq(u16x8::splat(0x2067))).any()
}

#[inline(always)]
pub fn simd_unpack(s: u8x16) -> (u16x8, u16x8) {
    unsafe {
        let first: u8x16 = simd_shuffle16(
            s,
            u8x16::splat(0),
            [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23],
        );
        let second: u8x16 = simd_shuffle16(
            s,
            u8x16::splat(0),
            [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31],
        );
        (::std::mem::transmute_copy(&first), ::std::mem::transmute_copy(&second))
    }
}

cfg_if! {
    if #[cfg(target_feature = "sse2")] {
        extern "platform-intrinsic" {
            fn x86_mm_packus_epi16(x: i16x8, y: i16x8) -> u8x16;
        }

        #[inline(always)]
        pub fn simd_pack(a: u16x8, b: u16x8) -> u8x16 {
            unsafe {
                let first: i16x8 = ::std::mem::transmute_copy(&a);
                let second: i16x8 = ::std::mem::transmute_copy(&b);
                x86_mm_packus_epi16(first, second)
            }
        }
    } else {
        #[inline(always)]
        pub fn simd_pack(a: u16x8, b: u16x8) -> u8x16 {
            unsafe {
                let first: u8x16 = ::std::mem::transmute_copy(&a);
                let second: u8x16 = ::std::mem::transmute_copy(&b);
                simd_shuffle16(
                    first,
                    second,
                    [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30],
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack() {
        let ascii: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let basic_latin: [u16; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70,
                                      0x71, 0x72, 0x73, 0x74, 0x75, 0x76];
        let simd = unsafe { load16_unaligned(ascii.as_ptr()) };
        let mut vec = Vec::with_capacity(16);
        vec.resize(16, 0u16);
        let (first, second) = simd_unpack(simd);
        let ptr = vec.as_mut_ptr();
        unsafe {
            store8_unaligned(ptr, first);
            store8_unaligned(ptr.offset(8), second);
        }
        assert_eq!(&vec[..], &basic_latin[..]);
    }

    #[test]
    fn test_simd_is_basic_latin_success() {
        let ascii: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let basic_latin: [u16; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70,
                                      0x71, 0x72, 0x73, 0x74, 0x75, 0x76];
        let first = unsafe { load8_unaligned(basic_latin.as_ptr()) };
        let second = unsafe { load8_unaligned(basic_latin.as_ptr().offset(8)) };
        let mut vec = Vec::with_capacity(16);
        vec.resize(16, 0u8);
        let ptr = vec.as_mut_ptr();
        assert!(simd_is_basic_latin(first | second));
        unsafe {
            store16_unaligned(ptr, simd_pack(first, second));
        }
        assert_eq!(&vec[..], &ascii[..]);
    }

    #[test]
    fn test_simd_is_basic_latin_c0() {
        let input: [u16; 16] = [0x61, 0x62, 0x63, 0x81, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                                0x72, 0x73, 0x74, 0x75, 0x76];
        let first = unsafe { load8_unaligned(input.as_ptr()) };
        let second = unsafe { load8_unaligned(input.as_ptr().offset(8)) };
        assert!(!simd_is_basic_latin(first | second));
    }

    #[test]
    fn test_simd_is_basic_latin_0fff() {
        let input: [u16; 16] = [0x61, 0x62, 0x63, 0x0FFF, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70,
                                0x71, 0x72, 0x73, 0x74, 0x75, 0x76];
        let first = unsafe { load8_unaligned(input.as_ptr()) };
        let second = unsafe { load8_unaligned(input.as_ptr().offset(8)) };
        assert!(!simd_is_basic_latin(first | second));
    }

    #[test]
    fn test_simd_is_basic_latin_ffff() {
        let input: [u16; 16] = [0x61, 0x62, 0x63, 0xFFFF, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70,
                                0x71, 0x72, 0x73, 0x74, 0x75, 0x76];
        let first = unsafe { load8_unaligned(input.as_ptr()) };
        let second = unsafe { load8_unaligned(input.as_ptr().offset(8)) };
        assert!(!simd_is_basic_latin(first | second));
    }

    #[test]
    fn test_simd_is_ascii_success() {
        let ascii: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let simd = unsafe { load16_unaligned(ascii.as_ptr()) };
        assert!(simd_is_ascii(simd));
    }

    #[test]
    fn test_simd_is_ascii_failure() {
        let input: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let simd = unsafe { load16_unaligned(input.as_ptr()) };
        assert!(!simd_is_ascii(simd));
    }

    #[cfg(target_feature = "sse2")]
    #[test]
    fn test_check_ascii() {
        let input: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let simd = unsafe { load16_unaligned(input.as_ptr()) };
        let mask = mask_ascii(simd);
        assert_ne!(mask, 0);
        assert_eq!(mask.trailing_zeros(), 4);
    }

    #[test]
    fn test_alu() {
        let input: [u8; 16] = [0x61, 0x62, 0x63, 0x64, 0x81, 0x66, 0x67, 0x68, 0x69, 0x70, 0x71,
                               0x72, 0x73, 0x74, 0x75, 0x76];
        let mut alu = 0u64;
        unsafe {
            ::std::ptr::copy_nonoverlapping(input.as_ptr(), &mut alu as *mut u64 as *mut u8, 8);
        }
        let masked = alu & 0x8080808080808080;
        assert_eq!(masked.trailing_zeros(), 39);
    }

}
