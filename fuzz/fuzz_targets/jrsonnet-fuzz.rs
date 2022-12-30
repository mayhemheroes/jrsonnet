#![no_main]

use jrsonnet_parser::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|code: String| {
	let _ = parse(
		&code.to_string(),
		&ParserSettings {
			file_name: Source::new_virtual(IStr::from("<test>"), IStr::from(code)),
		},
	);
});
