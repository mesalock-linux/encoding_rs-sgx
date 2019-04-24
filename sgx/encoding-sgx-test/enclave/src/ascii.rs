use encoding_rs::ascii::*;
use std::prelude::v1::*;

macro_rules! test_ascii {
    ($test_name:ident, $fn_tested:ident, $src_unit:ty, $dst_unit:ty) => {
        //#[test]
        pub fn $test_name() {
            let mut src: Vec<$src_unit> = Vec::with_capacity(32);
            let mut dst: Vec<$dst_unit> = Vec::with_capacity(32);
            for i in 0..32 {
                src.clear();
                dst.clear();
                dst.resize(32, 0);
                for j in 0..32 {
                    let c = if i == j { 0xAA } else { j + 0x40 };
                    src.push(c as $src_unit);
                }
                match unsafe { $fn_tested(src.as_ptr(), dst.as_mut_ptr(), 32) } {
                    None => unreachable!("Should always find non-ASCII"),
                    Some((non_ascii, num_ascii)) => {
                        assert_eq!(non_ascii, 0xAA);
                        assert_eq!(num_ascii, i);
                        for j in 0..i {
                            assert_eq!(dst[j], (j + 0x40) as $dst_unit);
                        }
                    }
                }
            }
        }
    };
}

test_ascii!(test_ascii_to_ascii, ascii_to_ascii, u8, u8);
test_ascii!(test_ascii_to_basic_latin, ascii_to_basic_latin, u8, u16);
test_ascii!(test_basic_latin_to_ascii, basic_latin_to_ascii, u16, u8);
