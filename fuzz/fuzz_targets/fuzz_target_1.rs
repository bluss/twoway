#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate twoway;

use std::mem::transmute;

// This fuzzing target splits data into two parts, and asserts that
// the result of str::find and twoway::find_str agree when passed
// the two &str parts as their parameters.
//
// A precondition is that the data is valid utf-8 (fuzzer is configured to use
// ascii only).

fuzz_target!(|data: &[u8]| {
    if data.len() > 2 {
        // use first 2 bytes as split point
        let (a, b) = (data[0] as usize, data[1] as usize);
        let (_, data) = data.split_at(2);

        // data is ascii only. We use a transmute to not put extra
        // code into the fuzzing, we don't want to fuzz utf-8 checking etc.
        let data: &str = unsafe { transmute(data) };
        let needle_split = ((a << 7) | b).min(data.len());
        let (needle, hay) = data.split_at(needle_split);
        assert_eq!(str::find(hay, needle), twoway::find_str(hay, needle));
    }
});
