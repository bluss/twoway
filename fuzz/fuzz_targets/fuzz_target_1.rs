#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate twoway;

use std::mem::transmute;

fuzz_target!(|data: &[u8]| {
    if data.len() > 2 {
        // use first 2 bytes as split point
        let (a, b) = (data[0] as usize, data[1] as usize);
        let (_, data) = data.split_at(2);

        // data is ascii only
        let data: &str = unsafe { transmute(data) };
        let needle_split = ((a << 7) | b).min(data.len());
        let (needle, hay) = data.split_at(needle_split);
        assert_eq!(str::find(hay, needle), twoway::find_str(hay, needle));
    }
});
