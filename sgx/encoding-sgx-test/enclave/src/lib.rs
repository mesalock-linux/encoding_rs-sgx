// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate encoding_rs;

mod testing;

mod shift_jis;
mod big5;
mod utf_16;
mod mem;
mod euc_jp;
mod test_labels_names;
mod x_user_defined;
mod ascii;
mod utf_8;
mod iso_2022_jp;
mod lib_test;
mod replacement;
mod single_byte;
mod gb18030;
mod euc_kr;
mod simd_funcs;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(shift_jis::test_shift_jis_decode,
                     shift_jis::test_shift_jis_encode,
                     shift_jis::test_shift_jis_decode_all,
                     shift_jis::test_shift_jis_encode_all,
                     shift_jis::test_shift_jis_half_width_katakana_length,
big5::test_big5_decode,
big5::test_big5_encode,
big5::test_big5_decode_all,
big5::test_big5_encode_all,
big5::test_big5_encode_from_two_low_surrogates,
utf_16::test_utf_16_decode,
utf_16::test_utf_16_encode,
utf_16::test_utf_16be_decode_one_by_one,
utf_16::test_utf_16le_decode_one_by_one,
utf_16::test_utf_16be_decode_three_at_a_time,
utf_16::test_utf_16le_decode_three_at_a_time,
utf_16::test_utf_16le_decode_bom_prefixed_split_byte_pair,
utf_16::test_utf_16be_decode_bom_prefixed_split_byte_pair,
utf_16::test_utf_16le_decode_bom_prefix,
utf_16::test_utf_16be_decode_bom_prefix,
utf_16::test_utf_16le_decode_near_end,
utf_16::test_utf_16be_decode_near_end,
mem::test_is_ascii_success,
mem::test_is_ascii_fail,
mem::test_is_basic_latin_success,
mem::test_is_basic_latin_fail,
mem::test_is_utf16_latin1_success,
mem::test_is_utf16_latin1_fail,
mem::test_is_str_latin1_success,
mem::test_is_str_latin1_fail,
mem::test_is_utf8_latin1_success,
mem::test_is_utf8_latin1_fail,
mem::test_is_utf8_latin1_invalid,
mem::test_convert_utf8_to_utf16,
mem::test_convert_str_to_utf16,
mem::test_convert_utf16_to_utf8_partial,
mem::test_convert_utf16_to_utf8,
mem::test_convert_latin1_to_utf16,
mem::test_convert_latin1_to_utf8_partial,
mem::test_convert_latin1_to_utf8,
mem::test_convert_utf8_to_latin1_lossy,
mem::test_convert_utf16_to_latin1_lossy,
mem::test_utf16_valid_up_to,
mem::test_ensure_utf16_validity,
mem::test_is_char_bidi,
mem::test_is_utf16_code_unit_bidi,
mem::test_is_str_bidi,
mem::test_is_utf8_bidi,
mem::test_is_utf16_bidi,
mem::test_check_str_for_latin1_and_bidi,
mem::test_check_utf8_for_latin1_and_bidi,
mem::test_check_utf16_for_latin1_and_bidi,
mem::test_is_char_bidi_thoroughly,
mem::test_is_utf16_code_unit_bidi_thoroughly,
mem::test_is_str_bidi_thoroughly,
mem::test_is_utf8_bidi_thoroughly,
mem::test_is_utf16_bidi_thoroughly,
mem::test_is_utf8_bidi_edge_cases,
mem::test_decode_latin1,
mem::test_encode_latin1_lossy,
euc_jp::test_euc_jp_decode,
euc_jp::test_euc_jp_encode,
euc_jp::test_jis0208_decode_all,
euc_jp::test_jis0208_encode_all,
euc_jp::test_jis0212_decode_all,
test_labels_names::test_all_labels,
x_user_defined::test_x_user_defined_decode,
x_user_defined::test_x_user_defined_encode,
x_user_defined::test_x_user_defined_from_two_low_surrogates,
ascii::test_ascii_to_ascii,
ascii::test_ascii_to_basic_latin,
ascii::test_basic_latin_to_ascii,
utf_8::test_utf8_decode,
utf_8::test_utf8_encode,
utf_8::test_encode_utf8_from_utf16_with_output_limit,
utf_8::test_utf8_max_length_from_utf16,
utf_8::test_decode_bom_prefixed_split_byte_triple,
utf_8::test_decode_bom_prefixed_split_byte_pair,
utf_8::test_decode_bom_prefix,
utf_8::test_tail,
iso_2022_jp::test_iso_2022_jp_decode,
iso_2022_jp::test_iso_2022_jp_encode,
iso_2022_jp::test_iso_2022_jp_decode_all,
iso_2022_jp::test_iso_2022_jp_encode_all,
iso_2022_jp::test_iso_2022_jp_half_width_katakana_length,
iso_2022_jp::test_iso_2022_jp_length_after_escape,
iso_2022_jp::test_iso_2022_jp_encode_from_two_low_surrogates,
lib_test::test_bom_sniffing,
lib_test::test_output_encoding,
lib_test::test_label_resolution,
lib_test::test_decode_valid_windows_1257_to_cow,
lib_test::test_decode_invalid_windows_1257_to_cow,
lib_test::test_decode_ascii_only_windows_1257_to_cow,
lib_test::test_decode_bomful_valid_utf8_as_windows_1257_to_cow,
lib_test::test_decode_bomful_invalid_utf8_as_windows_1257_to_cow,
lib_test::test_decode_bomful_valid_utf8_as_utf_8_to_cow,
lib_test::test_decode_bomful_invalid_utf8_as_utf_8_to_cow,
lib_test::test_decode_bomful_valid_utf8_as_utf_8_to_cow_with_bom_removal,
lib_test::test_decode_bomful_valid_utf8_as_windows_1257_to_cow_with_bom_removal,
lib_test::test_decode_valid_windows_1257_to_cow_with_bom_removal,
lib_test::test_decode_invalid_windows_1257_to_cow_with_bom_removal,
lib_test::test_decode_ascii_only_windows_1257_to_cow_with_bom_removal,
lib_test::test_decode_bomful_valid_utf8_to_cow_without_bom_handling,
lib_test::test_decode_bomful_invalid_utf8_to_cow_without_bom_handling,
lib_test::test_decode_valid_windows_1257_to_cow_without_bom_handling,
lib_test::test_decode_invalid_windows_1257_to_cow_without_bom_handling,
lib_test::test_decode_ascii_only_windows_1257_to_cow_without_bom_handling,
lib_test::test_decode_bomful_valid_utf8_to_cow_without_bom_handling_and_without_replacement,
lib_test::test_decode_bomful_invalid_utf8_to_cow_without_bom_handling_and_without_replacement,
lib_test::test_decode_valid_windows_1257_to_cow_without_bom_handling_and_without_replacement,
lib_test::test_decode_invalid_windows_1257_to_cow_without_bom_handling_and_without_replacement,
lib_test::test_decode_ascii_only_windows_1257_to_cow_without_bom_handling_and_without_replacement,
lib_test::test_encode_ascii_only_windows_1257_to_cow,
lib_test::test_encode_valid_windows_1257_to_cow,
lib_test::test_utf16_space_with_one_bom_byte,
lib_test::test_utf8_space_with_one_bom_byte,
lib_test::test_utf16_space_with_two_bom_bytes,
lib_test::test_utf8_space_with_two_bom_bytes,
lib_test::test_utf16_space_with_one_bom_byte_and_a_second_byte_in_same_call,
lib_test::test_too_short_buffer_with_iso_2022_jp_ascii_from_utf8,
lib_test::test_too_short_buffer_with_iso_2022_jp_roman_from_utf8,
lib_test::test_buffer_end_iso_2022_jp_from_utf8,
lib_test::test_too_short_buffer_with_iso_2022_jp_ascii_from_utf16,
lib_test::test_too_short_buffer_with_iso_2022_jp_roman_from_utf16,
lib_test::test_buffer_end_iso_2022_jp_from_utf16,
lib_test::test_hash,
lib_test::test_iso_2022_jp_ncr_extra_from_utf16,
lib_test::test_iso_2022_jp_ncr_extra_from_utf8,
lib_test::test_max_length_with_bom_to_utf8,
//lib_test::test_serde,
lib_test::test_is_single_byte,
replacement::test_replacement_decode,
replacement::test_replacement_encode,
single_byte::test_windows_1255_ca,
single_byte::test_ascii_punctuation,
single_byte::test_decode_malformed,
single_byte::test_encode_unmappables,
single_byte::test_encode_unpaired_surrogates,
single_byte::test_single_byte_from_two_low_surrogates,
single_byte::test_single_byte_decode,
single_byte::test_single_byte_encode,
gb18030::test_gb18030_decode,
gb18030::test_gb18030_encode,
gb18030::test_gbk_encode,
gb18030::test_gb18030_decode_all,
gb18030::test_gb18030_encode_all,
gb18030::test_gb18030_encode_from_utf16_max_length,
euc_kr::test_euc_kr_decode,
euc_kr::test_euc_kr_encode,
euc_kr::test_euc_kr_decode_all,
euc_kr::test_euc_kr_encode_all,
euc_kr::test_euc_kr_encode_from_two_low_surrogates,
simd_funcs::test_unpack,
simd_funcs::test_simd_is_basic_latin_success,
simd_funcs::test_simd_is_basic_latin_c0,
simd_funcs::test_simd_is_basic_latin_0fff,
simd_funcs::test_simd_is_basic_latin_ffff,
simd_funcs::test_simd_is_ascii_success,
simd_funcs::test_simd_is_ascii_failure,
simd_funcs::test_check_ascii,
simd_funcs::test_alu
);

    sgx_status_t::SGX_SUCCESS
}
