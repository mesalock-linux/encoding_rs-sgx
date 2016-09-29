// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The C API for encoding_rs.
//!
//! # Mapping from Rust
//!
//! ## Naming convention
//!
//! The wrapper function for each method has a name that starts with the name
//! of the struct lower-cased, followed by an underscore and ends with the
//! name of the method.
//!
//! For example, `Encoding::for_name()` is wrapped as `encoding_for_name()`.
//!
//! ## Arguments
//!
//! Functions that wrap non-static methods take the `self` object as their
//! first argument.
//!
//! Slice argument `foo` is decomposed into a pointer `foo` and a length
//! `foo_len`.
//!
//! ## Return values
//!
//! Multiple return values become out-params. When an out-param is
//! length-related, `foo_len` for a slice becomes a pointer in order to become
//! an in/out-param.
//!
//! `DecoderResult`, `EncoderResult` and `CoderResult` become `uint32_t`.
//! `InputEmpty` becomes `INPUT_EMPTY`. `OutputFull` becomes `OUTPUT_FULL`.
//! `Unmappable` becomes the scalar value of the unmappable character.
//! `Malformed` becomes a number whose lowest 8 bits, which can have the decimal
//! value 0, 1, 2 or 3, indicate the number of bytes that were consumed after
//! the malformed sequence and whose next-lowest 8 bits, when shifted right by
//! 8 indicate the length of the malformed byte sequence (possible decimal
//! values 1, 2 and 3).

use super::*;

/// Return value for `*_decode_*` and `*_encode_*` functions that indicates that
/// the input has been exhausted.
///
/// (This is zero as a micro optimization. U+0000 is never unmappable and
/// malformed sequences always have a positive length.)
pub const INPUT_EMPTY: u32 = 0;

/// Return value for `*_decode_*` and `*_encode_*` functions that indicates that
/// the output space has been exhausted.
pub const OUTPUT_FULL: u32 = 0xFFFFFFFF;

/// The minimum length of buffers that may be passed to `encoding_name()`.
pub const ENCODING_NAME_MAX_LENGTH: usize = super::LONGEST_NAME_LENGTH;

/// Newtype for `*const Encoding` in order to be able to implement `Sync` for
/// it.
pub struct ConstEncoding(*const Encoding);

/// Required for `static` fields.
unsafe impl Sync for ConstEncoding {}

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate-encoding-data.py

/// The Big5 encoding.
#[no_mangle]
pub static BIG5_ENCODING: ConstEncoding = ConstEncoding(BIG5);

/// The EUC-JP encoding.
#[no_mangle]
pub static EUC_JP_ENCODING: ConstEncoding = ConstEncoding(EUC_JP);

/// The EUC-KR encoding.
#[no_mangle]
pub static EUC_KR_ENCODING: ConstEncoding = ConstEncoding(EUC_KR);

/// The GBK encoding.
#[no_mangle]
pub static GBK_ENCODING: ConstEncoding = ConstEncoding(GBK);

/// The IBM866 encoding.
#[no_mangle]
pub static IBM866_ENCODING: ConstEncoding = ConstEncoding(IBM866);

/// The ISO-2022-JP encoding.
#[no_mangle]
pub static ISO_2022_JP_ENCODING: ConstEncoding = ConstEncoding(ISO_2022_JP);

/// The ISO-8859-10 encoding.
#[no_mangle]
pub static ISO_8859_10_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_10);

/// The ISO-8859-13 encoding.
#[no_mangle]
pub static ISO_8859_13_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_13);

/// The ISO-8859-14 encoding.
#[no_mangle]
pub static ISO_8859_14_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_14);

/// The ISO-8859-15 encoding.
#[no_mangle]
pub static ISO_8859_15_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_15);

/// The ISO-8859-16 encoding.
#[no_mangle]
pub static ISO_8859_16_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_16);

/// The ISO-8859-2 encoding.
#[no_mangle]
pub static ISO_8859_2_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_2);

/// The ISO-8859-3 encoding.
#[no_mangle]
pub static ISO_8859_3_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_3);

/// The ISO-8859-4 encoding.
#[no_mangle]
pub static ISO_8859_4_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_4);

/// The ISO-8859-5 encoding.
#[no_mangle]
pub static ISO_8859_5_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_5);

/// The ISO-8859-6 encoding.
#[no_mangle]
pub static ISO_8859_6_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_6);

/// The ISO-8859-7 encoding.
#[no_mangle]
pub static ISO_8859_7_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_7);

/// The ISO-8859-8 encoding.
#[no_mangle]
pub static ISO_8859_8_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_8);

/// The ISO-8859-8-I encoding.
#[no_mangle]
pub static ISO_8859_8_I_ENCODING: ConstEncoding = ConstEncoding(ISO_8859_8_I);

/// The KOI8-R encoding.
#[no_mangle]
pub static KOI8_R_ENCODING: ConstEncoding = ConstEncoding(KOI8_R);

/// The KOI8-U encoding.
#[no_mangle]
pub static KOI8_U_ENCODING: ConstEncoding = ConstEncoding(KOI8_U);

/// The Shift_JIS encoding.
#[no_mangle]
pub static SHIFT_JIS_ENCODING: ConstEncoding = ConstEncoding(SHIFT_JIS);

/// The UTF-16BE encoding.
#[no_mangle]
pub static UTF_16BE_ENCODING: ConstEncoding = ConstEncoding(UTF_16BE);

/// The UTF-16LE encoding.
#[no_mangle]
pub static UTF_16LE_ENCODING: ConstEncoding = ConstEncoding(UTF_16LE);

/// The UTF-8 encoding.
#[no_mangle]
pub static UTF_8_ENCODING: ConstEncoding = ConstEncoding(UTF_8);

/// The gb18030 encoding.
#[no_mangle]
pub static GB18030_ENCODING: ConstEncoding = ConstEncoding(GB18030);

/// The macintosh encoding.
#[no_mangle]
pub static MACINTOSH_ENCODING: ConstEncoding = ConstEncoding(MACINTOSH);

/// The replacement encoding.
#[no_mangle]
pub static REPLACEMENT_ENCODING: ConstEncoding = ConstEncoding(REPLACEMENT);

/// The windows-1250 encoding.
#[no_mangle]
pub static WINDOWS_1250_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1250);

/// The windows-1251 encoding.
#[no_mangle]
pub static WINDOWS_1251_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1251);

/// The windows-1252 encoding.
#[no_mangle]
pub static WINDOWS_1252_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1252);

/// The windows-1253 encoding.
#[no_mangle]
pub static WINDOWS_1253_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1253);

/// The windows-1254 encoding.
#[no_mangle]
pub static WINDOWS_1254_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1254);

/// The windows-1255 encoding.
#[no_mangle]
pub static WINDOWS_1255_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1255);

/// The windows-1256 encoding.
#[no_mangle]
pub static WINDOWS_1256_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1256);

/// The windows-1257 encoding.
#[no_mangle]
pub static WINDOWS_1257_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1257);

/// The windows-1258 encoding.
#[no_mangle]
pub static WINDOWS_1258_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_1258);

/// The windows-874 encoding.
#[no_mangle]
pub static WINDOWS_874_ENCODING: ConstEncoding = ConstEncoding(WINDOWS_874);

/// The x-mac-cyrillic encoding.
#[no_mangle]
pub static X_MAC_CYRILLIC_ENCODING: ConstEncoding = ConstEncoding(X_MAC_CYRILLIC);

/// The x-user-defined encoding.
#[no_mangle]
pub static X_USER_DEFINED_ENCODING: ConstEncoding = ConstEncoding(X_USER_DEFINED);

// END GENERATED CODE

impl CoderResult {
    fn as_u32(&self) -> u32 {
        match self {
            &CoderResult::InputEmpty => INPUT_EMPTY,
            &CoderResult::OutputFull => OUTPUT_FULL,
        }
    }
}

impl DecoderResult {
    fn as_u32(&self) -> u32 {
        match self {
            &DecoderResult::InputEmpty => INPUT_EMPTY,
            &DecoderResult::OutputFull => OUTPUT_FULL,
            &DecoderResult::Malformed(bad, good) => ((good as u32) << 8) | (bad as u32),
        }
    }
}

impl EncoderResult {
    fn as_u32(&self) -> u32 {
        match self {
            &EncoderResult::InputEmpty => INPUT_EMPTY,
            &EncoderResult::OutputFull => OUTPUT_FULL,
            &EncoderResult::Unmappable(c) => c as u32,
        }
    }
}

fn option_to_ptr(opt: Option<&'static Encoding>) -> *const Encoding {
    match opt {
        None => ::std::ptr::null(),
        Some(e) => e,
    }
}

/// Implements the
/// [_get an encoding_](https://encoding.spec.whatwg.org/#concept-encoding-get)
/// algorithm.
///
/// If, after ASCII-lowercasing and removing leading and trailing
/// whitespace, the argument matches a label defined in the Encoding
/// Standard, `const Encoding*` representing the corresponding
/// encoding is returned. If there is no match, `NULL` is returned.
///
/// The argument buffer can be in any ASCII-compatible encoding. It is not
/// required to be UTF-8.
///
/// # Undefined behavior
///
/// UB ensues if `label` and `label_len` don't designate a valid memory block.
#[no_mangle]
pub unsafe extern "C" fn encoding_for_label(label: *const u8, label_len: usize) -> *const Encoding {
    let label_slice = ::std::slice::from_raw_parts(label, label_len);
    option_to_ptr(Encoding::for_label(label_slice))
}

/// This function behaves the same as `encoding_for_label()`, except when
/// `encoding_for_label()` would return `REPLACEMENT_ENCODING`, this method
/// returns `NULL` instead.
///
/// This method is useful in scenarios where a fatal error is required
/// upon invalid label, because in those cases the caller typically wishes
/// to treat the labels that map to the replacement encoding as fatal
/// errors, too.
///
/// # Undefined behavior
///
/// UB ensues if `label` and `label_len` don't designate a valid memory block.
#[no_mangle]
pub unsafe extern "C" fn encoding_for_label_no_replacement(label: *const u8,
                                                           label_len: usize)
                                                           -> *const Encoding {
    let label_slice = ::std::slice::from_raw_parts(label, label_len);
    option_to_ptr(Encoding::for_label_no_replacement(label_slice))
}

/// If the argument matches exactly (case-sensitively; no whitespace
/// removal performed) the name of an encoding, returns
/// `const Encoding*` representing that encoding. Otherwise,
/// return `NULL`.
///
/// The motivating use case for this function is interoperability with
/// legacy Gecko code that represents encodings as name string instead of
/// type-safe `Encoding` objects. Using this function for other purposes is
/// most likely the wrong thing to do.
///
/// # Undefined behavior
///
/// UB ensues if `name` and `name_len` don't designate a valid memory block.
#[no_mangle]
pub unsafe extern "C" fn encoding_for_name(name: *const u8, name_len: usize) -> *const Encoding {
    let name_slice = ::std::slice::from_raw_parts(name, name_len);
    option_to_ptr(Encoding::for_name(name_slice))
}

/// Performs non-incremental BOM sniffing.
///
/// The argument must either be a buffer representing the entire input
/// stream (non-streaming case) or a buffer representing at least the first
/// three bytes of the input stream (streaming case).
///
/// Returns `UTF_8_ENCODING`, `UTF_16LE_ENCODING` or `UTF_16BE_ENCODING` if the
/// argument starts with the UTF-8, UTF-16LE or UTF-16BE BOM or `None`
/// otherwise.
///
/// # Undefined behavior
///
/// UB ensues if `buffer` and `buffer_len` don't designate a valid memory block.
#[no_mangle]
pub unsafe extern "C" fn encoding_for_bom(buffer: *const u8, buffer_len: usize) -> *const Encoding {
    let buffer_slice = ::std::slice::from_raw_parts(buffer, buffer_len);
    option_to_ptr(Encoding::for_bom(buffer_slice))
}

/// Writes the name of the given `Encoding` to a caller-supplied buffer as ASCII
/// and returns the number of bytes / ASCII characters written.
///
/// The output is not null-terminated.
///
/// The caller _MUST_ ensure that `name_out` points to a buffer whose length
/// is at least `ENCODING_NAME_MAX_LENGTH` bytes.
///
/// # Undefined behavior
///
/// UB ensues if either argument is `NULL` or if `name_out` doesn't point to
/// a valid block of memory whose length is at least `ENCODING_NAME_MAX_LENGTH`
/// bytes.
#[no_mangle]
pub unsafe extern "C" fn encoding_name(encoding: *const Encoding, name_out: *mut u8) -> usize {
    let bytes = (*encoding).name().as_bytes();
    ::std::ptr::copy_nonoverlapping(bytes.as_ptr(), name_out, bytes.len());
    bytes.len()
}

/// Checks whether the _output encoding_ of this encoding can encode every
/// Unicode scalar. (Only true if the output encoding is UTF-8.)
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_can_encode_everything(encoding: *const Encoding) -> bool {
    (*encoding).can_encode_everything()
}

/// Checks whether the bytes 0x00...0x7F map exclusively to the characters
/// U+0000...U+007F and vice versa.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_is_ascii_compatible(encoding: *const Encoding) -> bool {
    (*encoding).is_ascii_compatible()
}

/// Returns the _output encoding_ of this encoding. This is UTF-8 for
/// UTF-16BE, UTF-16LE and replacement and the encoding itself otherwise.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_output_encoding(encoding: *const Encoding) -> *const Encoding {
    (*encoding).output_encoding()
}

/// Allocates a new `Decoder` for the given `Encoding` on the heap with BOM
/// sniffing enabled and returns a pointer to the newly-allocated `Decoder`.
///
/// BOM sniffing may cause the returned decoder to morph into a decoder
/// for UTF-8, UTF-16LE or UTF-16BE instead of this encoding.
///
/// Once the allocated `Decoder` is no longer needed, the caller _MUST_
/// deallocate it by passing the pointer returned by this function to
/// `decoder_free()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder(encoding: *const Encoding) -> *mut Decoder {
    Box::into_raw(Box::new((*encoding).new_decoder()))
}

/// Allocates a new `Decoder` for the given `Encoding` on the heap with BOM
/// removal and returns a pointer to the newly-allocated `Decoder`.
///
/// If the input starts with bytes that are the BOM for this encoding,
/// those bytes are removed. However, the decoder never morphs into a
/// decoder for another encoding: A BOM for another encoding is treated as
/// (potentially malformed) input to the decoding algorithm for this
/// encoding.
///
/// Once the allocated `Decoder` is no longer needed, the caller _MUST_
/// deallocate it by passing the pointer returned by this function to
/// `decoder_free()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder_with_bom_removal(encoding: *const Encoding)
                                                               -> *mut Decoder {
    Box::into_raw(Box::new((*encoding).new_decoder_with_bom_removal()))
}

/// Allocates a new `Decoder` for the given `Encoding` on the heap with BOM
/// handling disabled and returns a pointer to the newly-allocated `Decoder`.
///
/// If the input starts with bytes that look like a BOM, those bytes are
/// not treated as a BOM. (Hence, the decoder never morphs into a decoder
/// for another encoding.)
///
/// _Note:_ If the caller has performed BOM sniffing on its own but has not
/// removed the BOM, the caller should use
/// `encoding_new_decoder_with_bom_removal()` instead of this function to cause
/// the BOM to be removed.
///
/// Once the allocated `Decoder` is no longer needed, the caller _MUST_
/// deallocate it by passing the pointer returned by this function to
/// `decoder_free()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder_without_bom_handling(encoding: *const Encoding)
                                                                   -> *mut Decoder {
    Box::into_raw(Box::new((*encoding).new_decoder_without_bom_handling()))
}

/// Allocates a new `Decoder` for the given `Encoding` into memory provided by
/// the caller with BOM sniffing enabled. (In practice, the target should
/// likely be a pointer previously returned by `encoding_new_decoder()`.)
///
/// Note: If the caller has already performed BOM sniffing but has
/// not removed the BOM, the caller should still use this function in
/// order to cause the BOM to be ignored.
///
/// # Undefined behavior
///
/// UB ensues if either argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder_into(encoding: *const Encoding,
                                                   decoder: *mut Decoder) {
    *decoder = (*encoding).new_decoder();
}

/// Allocates a new `Decoder` for the given `Encoding` into memory provided by
/// the caller with BOM removal.
///
/// If the input starts with bytes that are the BOM for this encoding,
/// those bytes are removed. However, the decoder never morphs into a
/// decoder for another encoding: A BOM for another encoding is treated as
/// (potentially malformed) input to the decoding algorithm for this
/// encoding.
///
/// Once the allocated `Decoder` is no longer needed, the caller _MUST_
/// deallocate it by passing the pointer returned by this function to
/// `decoder_free()`.
///
/// # Undefined behavior
///
/// UB ensues if either argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder_with_bom_removal_into(encoding: *const Encoding,
                                                                    decoder: *mut Decoder) {
    *decoder = (*encoding).new_decoder_with_bom_removal();
}

/// Allocates a new `Decoder` for the given `Encoding` into memory provided by
/// the caller with BOM handling disabled.
///
/// If the input starts with bytes that look like a BOM, those bytes are
/// not treated as a BOM. (Hence, the decoder never morphs into a decoder
/// for another encoding.)
///
/// _Note:_ If the caller has performed BOM sniffing on its own but has not
/// removed the BOM, the caller should use
/// `encoding_new_decoder_with_bom_removal_into()` instead of this function to
/// cause the BOM to be removed.
///
/// # Undefined behavior
///
/// UB ensues if either argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_decoder_without_bom_handling_into(encoding: *const Encoding,
                                                                        decoder: *mut Decoder) {
    *decoder = (*encoding).new_decoder_without_bom_handling();
}

/// Allocates a new `Encoder` for the given `Encoding` on the heap and returns a
/// pointer to the newly-allocated `Encoder`. (Exception, if the `Encoding` is
/// `replacement`, a new `Decoder` for UTF-8 is instantiated (and that
/// `Decoder` reports `UTF_8` as its `Encoding`).
///
/// Once the allocated `Encoder` is no longer needed, the caller _MUST_
/// deallocate it by passing the pointer returned by this function to
/// `encoder_free()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_encoder(encoding: *const Encoding) -> *mut Encoder {
    Box::into_raw(Box::new((*encoding).new_encoder()))
}

/// Allocates a new `Encoder` for the given `Encoding` into memory provided by
/// the caller. (In practice, the target should likely be a pointer previously
/// returned by `encoding_new_encoder()`.)
///
/// # Undefined behavior
///
/// UB ensues if either argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoding_new_encoder_into(encoding: *const Encoding,
                                                   encoder: *mut Encoder) {
    *encoder = (*encoding).new_encoder();
}

/// Deallocates a `Decoder` previously allocated by `encoding_new_decoder()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn decoder_free(decoder: *mut Decoder) {
    let _ = Box::from_raw(decoder);
}

/// The `Encoding` this `Decoder` is for.
///
/// BOM sniffing can change the return value of this method during the life
/// of the decoder.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn decoder_encoding(decoder: *const Decoder) -> *const Encoding {
    (*decoder).encoding()
}

/// Query the worst-case UTF-16 output size (with or without replacement).
///
/// Returns the size of the output buffer in UTF-16 code units (`char16_t`)
/// that will not overflow given the current state of the decoder and
/// `byte_length` number of additional input bytes.
///
/// Since the REPLACEMENT CHARACTER fits into one UTF-16 code unit, the
/// return value of this method applies also in the
/// `_without_replacement` case.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn decoder_max_utf16_buffer_length(decoder: *const Decoder,
                                                         u16_length: usize)
                                                         -> usize {
    (*decoder).max_utf16_buffer_length(u16_length)
}

/// Query the worst-case UTF-8 output size _without replacement_.
///
/// Returns the size of the output buffer in UTF-8 code units (`uint8_t`)
/// that will not overflow given the current state of the decoder and
/// `byte_length` number of additional input bytes when decoding without
/// replacement error handling.
///
/// Note that this value may be too small for the `_with_replacement` case.
/// Use `decoder_max_utf8_buffer_length()` for that case.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn decoder_max_utf8_buffer_length_without_replacement(decoder: *const Decoder,
                                                                            byte_length: usize)
                                                                            -> usize {
    (*decoder).max_utf8_buffer_length_without_replacement(byte_length)
}

/// Query the worst-case UTF-8 output size _with replacement_.
///
/// Returns the size of the output buffer in UTF-8 code units (`uint8_t`)
/// that will not overflow given the current state of the decoder and
/// `byte_length` number of additional input bytes when decoding with
/// errors handled by outputting a REPLACEMENT CHARACTER for each malformed
/// sequence.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn decoder_max_utf8_buffer_length(decoder: *const Decoder,
                                                        byte_length: usize)
                                                        -> usize {
    (*decoder).max_utf8_buffer_length(byte_length)
}

/// Incrementally decode a byte stream into UTF-16 _without replacement_.
///
/// See the top-level FFI documentation for documentation for how the
/// `decoder_decode_*` functions are mapped from Rust and the documentation
/// for the [`Decoder`][1] struct for the semantics.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory or `dst` and `dst_len` don't designate a valid block
/// of memory.
///
/// [1]: ../struct.Decoder.html
#[no_mangle]
pub unsafe extern "C" fn decoder_decode_to_utf16_without_replacement(decoder: *mut Decoder,
                                                                     src: *const u8,
                                                                     src_len: *mut usize,
                                                                     dst: *mut u16,
                                                                     dst_len: *mut usize,
                                                                     last: bool)
                                                                     -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written) = (*decoder).decode_to_utf16_without_replacement(src_slice,
                                                                                 dst_slice,
                                                                                 last);
    *src_len = read;
    *dst_len = written;
    result.as_u32()
}

/// Incrementally decode a byte stream into UTF-8 _without replacement_.
///
/// See the top-level FFI documentation for documentation for how the
/// `decoder_decode_*` functions are mapped from Rust and the documentation
/// for the [`Decoder`][1] struct for the semantics.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory or `dst` and `dst_len` don't designate a valid block
/// of memory.
///
/// [1]: ../struct.Decoder.html
#[no_mangle]
pub unsafe extern "C" fn decoder_decode_to_utf8_without_replacement(decoder: *mut Decoder,
                                                                    src: *const u8,
                                                                    src_len: *mut usize,
                                                                    dst: *mut u8,
                                                                    dst_len: *mut usize,
                                                                    last: bool)
                                                                    -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written) = (*decoder).decode_to_utf8_without_replacement(src_slice,
                                                                                dst_slice,
                                                                                last);
    *src_len = read;
    *dst_len = written;
    result.as_u32()
}

/// Incrementally decode a byte stream into UTF-16 with malformed sequences
/// replaced with the REPLACEMENT CHARACTER.
///
/// See the top-level FFI documentation for documentation for how the
/// `decoder_decode_*` functions are mapped from Rust and the documentation
/// for the [`Decoder`][1] struct for the semantics.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory, `dst` and `dst_len` don't designate a valid block
/// of memory or `had_replacements` is `NULL`.
///
/// [1]: ../struct.Decoder.html
#[no_mangle]
pub unsafe extern "C" fn decoder_decode_to_utf16(decoder: *mut Decoder,
                                                 src: *const u8,
                                                 src_len: *mut usize,
                                                 dst: *mut u16,
                                                 dst_len: *mut usize,
                                                 last: bool,
                                                 had_replacements: *mut bool)
                                                 -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written, replaced) = (*decoder).decode_to_utf16(src_slice, dst_slice, last);
    *src_len = read;
    *dst_len = written;
    *had_replacements = replaced;
    result.as_u32()
}

/// Incrementally decode a byte stream into UTF-8 with malformed sequences
/// replaced with the REPLACEMENT CHARACTER.
///
/// See the top-level FFI documentation for documentation for how the
/// `decoder_decode_*` functions are mapped from Rust and the documentation
/// for the [`Decoder`][1] struct for the semantics.
///
/// # Undefined behavior
///
/// UB ensues if `decoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory, `dst` and `dst_len` don't designate a valid block
/// of memory or `had_replacements` is `NULL`.
///
/// [1]: ../struct.Decoder.html
#[no_mangle]
pub unsafe extern "C" fn decoder_decode_to_utf8(decoder: *mut Decoder,
                                                src: *const u8,
                                                src_len: *mut usize,
                                                dst: *mut u8,
                                                dst_len: *mut usize,
                                                last: bool,
                                                had_replacements: *mut bool)
                                                -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written, replaced) = (*decoder).decode_to_utf8(src_slice, dst_slice, last);
    *src_len = read;
    *dst_len = written;
    *had_replacements = replaced;
    result.as_u32()
}

/// Deallocates an `Encoder` previously allocated by `encoding_new_encoder()`.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoder_free(encoder: *mut Encoder) {
    let _ = Box::from_raw(encoder);
}

/// The `Encoding` this `Encoder` is for.
///
/// # Undefined behavior
///
/// UB ensues if the argument is `NULL`.
#[no_mangle]
pub unsafe extern "C" fn encoder_encoding(encoder: *const Encoder) -> *const Encoding {
    (*encoder).encoding()
}

/// Query the worst-case output size when encoding from UTF-16 without
/// replacement.
///
/// Returns the size of the output buffer in bytes that will not overflow
/// given the current state of the encoder and `u16_length` number of
/// additional input code units.
#[no_mangle]
pub unsafe extern "C" fn encoder_max_buffer_length_from_utf16_without_replacement(encoder: *const Encoder,
                                                              u16_length: usize)
                                                              -> usize {
    (*encoder).max_buffer_length_from_utf16_without_replacement(u16_length)
}

/// Query the worst-case output size when encoding from UTF-8 without
/// replacement.
///
/// Returns the size of the output buffer in bytes that will not overflow
/// given the current state of the encoder and `byte_length` number of
/// additional input code units.
#[no_mangle]
pub unsafe extern "C" fn encoder_max_buffer_length_from_utf8_without_replacement(encoder: *const Encoder,
                                                             byte_length: usize)
                                                             -> usize {
    (*encoder).max_buffer_length_from_utf8_without_replacement(byte_length)
}

/// Query the worst-case output size when encoding from UTF-16 with
/// replacement.
///
/// Returns the size of the output buffer in bytes that will not overflow
/// given the current state of the encoder and `u16_length` number of
/// additional input code units if there are no unmappable characters in
/// the input.
#[no_mangle]
pub unsafe extern "C" fn encoder_max_buffer_length_from_utf16_if_no_unmappables
    (encoder: *const Encoder,
     u16_length: usize)
     -> usize {
    (*encoder).max_buffer_length_from_utf16_if_no_unmappables(u16_length)
}

/// Query the worst-case output size when encoding from UTF-8 with
/// replacement.
///
/// Returns the size of the output buffer in bytes that will not overflow
/// given the current state of the encoder and `byte_length` number of
/// additional input code units if there are no unmappable characters in
/// the input.
#[no_mangle]
pub unsafe extern "C" fn encoder_max_buffer_length_from_utf8_if_no_unmappables
    (encoder: *const Encoder,
     byte_length: usize)
     -> usize {
    (*encoder).max_buffer_length_from_utf8_if_no_unmappables(byte_length)
}

/// Incrementally encode into byte stream from UTF-16 _without replacement_.
///
/// See the top-level FFI documentation for documentation for how the
/// `encoder_encode_*` functions are mapped from Rust and the documentation
/// for the [`Encoder`][1] struct for the semantics.
///
/// # Undefined behavior
///
/// UB ensues if `encoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory or `dst` and `dst_len` don't designate a valid block
/// of memory.
///
/// [1]: ../struct.Encoder.html
#[no_mangle]
pub unsafe extern "C" fn encoder_encode_from_utf16_without_replacement(encoder: *mut Encoder,
                                                                       src: *const u16,
                                                                       src_len: *mut usize,
                                                                       dst: *mut u8,
                                                                       dst_len: *mut usize,
                                                                       last: bool)
                                                                       -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written) = (*encoder).encode_from_utf16_without_replacement(src_slice,
                                                                                   dst_slice,
                                                                                   last);
    *src_len = read;
    *dst_len = written;
    result.as_u32()
}

/// Incrementally encode into byte stream from UTF-8 _without replacement_.
///
/// See the top-level FFI documentation for documentation for how the
/// `encoder_encode_*` functions are mapped from Rust and the documentation
/// for the [`Encoder`][1] struct for the semantics.
///
/// The input absolutely _MUST_ be valid UTF-8 or the behavior is memory-unsafe!
/// If in doubt, check the validity of input before using!
///
/// # Undefined behavior
///
/// UB ensues if `encoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory, `dst` and `dst_len` don't designate a valid block
/// of memory or the input is not valid UTF-8.
///
/// [1]: ../struct.Encoder.html
#[no_mangle]
pub unsafe extern "C" fn encoder_encode_from_utf8_without_replacement(encoder: *mut Encoder,
                                                                      src: *const u8,
                                                                      src_len: *mut usize,
                                                                      dst: *mut u8,
                                                                      dst_len: *mut usize,
                                                                      last: bool)
                                                                      -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let string = ::std::str::from_utf8_unchecked(src_slice);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written) = (*encoder).encode_from_utf8_without_replacement(string,
                                                                                  dst_slice,
                                                                                  last);
    *src_len = read;
    *dst_len = written;
    result.as_u32()
}

/// Incrementally encode into byte stream from UTF-16 with unmappable
/// characters replaced with HTML (decimal) numeric character references.
///
/// See the top-level FFI documentation for documentation for how the
/// `encoder_encode_*` functions are mapped from Rust and the documentation
/// for the [`Encoder`][1] struct for the semantics.
///
/// UB ensues if `encoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory, `dst` and `dst_len` don't designate a valid block
/// of memory or `had_replacements` is `NULL`.
///
/// [1]: ../struct.Encoder.html
#[no_mangle]
pub unsafe extern "C" fn encoder_encode_from_utf16(encoder: *mut Encoder,
                                                   src: *const u16,
                                                   src_len: *mut usize,
                                                   dst: *mut u8,
                                                   dst_len: *mut usize,
                                                   last: bool,
                                                   had_replacements: *mut bool)
                                                   -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written, replaced) = (*encoder)
                                                .encode_from_utf16(src_slice, dst_slice, last);
    *src_len = read;
    *dst_len = written;
    *had_replacements = replaced;
    result.as_u32()
}

/// Incrementally encode into byte stream from UTF-8 with unmappable
/// characters replaced with HTML (decimal) numeric character references.
///
/// The input absolutely _MUST_ be valid UTF-8 or the behavior is memory-unsafe!
/// If in doubt, check the validity of input before using!
///
/// See the top-level FFI documentation for documentation for how the
/// `encoder_encode_*` functions are mapped from Rust and the documentation
/// for the [`Encoder`][1] struct for the semantics.
///
/// UB ensues if `encoder` is `NULL`, `src` and `src_len` don't designate a
/// valid block of memory, `dst` and `dst_len` don't designate a valid block
/// of memory, `had_replacements` is `NULL` or the input is not valid UTF-8.
///
/// [1]: ../struct.Encoder.html
#[no_mangle]
pub unsafe extern "C" fn encoder_encode_from_utf8(encoder: *mut Encoder,
                                                  src: *const u8,
                                                  src_len: *mut usize,
                                                  dst: *mut u8,
                                                  dst_len: *mut usize,
                                                  last: bool,
                                                  had_replacements: *mut bool)
                                                  -> u32 {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let string = ::std::str::from_utf8_unchecked(src_slice);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (result, read, written, replaced) = (*encoder).encode_from_utf8(string, dst_slice, last);
    *src_len = read;
    *dst_len = written;
    *had_replacements = replaced;
    result.as_u32()
}
