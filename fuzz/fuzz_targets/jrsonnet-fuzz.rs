#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;
use jrsonnet_parser::*;
use std::borrow::Cow;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(&data[0..]) {
        Ok(in_string)=>{
            parse(in_string, &ParserSettings {
                file_name: Source::new_virtual(Cow::Borrowed("<test>")),
            });
        },
        Err(..)=>()
    }
});
