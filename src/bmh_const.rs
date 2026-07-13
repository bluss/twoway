//! Copyright 2015 - 2026 Ulrik Sverdrup "bluss"
//!
//! Boyer-Moore-Horspool string search in const Rust

const fn min_usize(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

/// Compute skip table
const fn bmh_skip(pat: &[u8], skip: &mut [u16; 256]) {
    let pat_skip = min_usize(pat.len(), u16::max_value() as usize) as u16;
    let mut i = 0;
    while i < skip.len() {
        skip[i] = pat_skip;
        i += 1;
    }

    let mut index = 0;
    while index + 1 < pat.len() {
        let byte = pat[index];
        skip[byte as usize] = min_usize(pat.len() - index - 1, u16::max_value() as usize) as u16;
        index += 1;
    }
}

const fn slice_get<T>(text: &[T], index: usize) -> Option<&T> {
    if index < text.len() {
        Some(&text[index])
    } else { None }

}

const fn slice_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] { return false; }
        i += 1;
    }
    true
}

/// Test if &text[start..start + pat.len()] == pat
/// Panics if &text[start..] is longer than pat.
const fn slice_subset_eq(text: &[u8], start: usize, pat: &[u8]) -> bool {
    let (_, tail) = text.split_at(start);
    let (mid, _) = tail.split_at(pat.len());
    slice_eq(mid, pat)
}

/// Boyer-Moore-Horspool substring search
///
/// Return first index of pattern match in the text, if any.
pub const fn find(text: &[u8], pat: &[u8]) -> Option<usize> {
    let mut skip = [0; 256];
    bmh_skip(pat, &mut skip);

    let pat_len = pat.len();

    if pat_len == 0 {
        return Some(0);
    }

    let pat_len_m1 = pat_len - 1;
    let pat_last = pat[pat_len - 1];

    // initialize j with search for first character
    let mut j = 0;
    while j < text.len() && text[j] != pat[0] {
        j += 1;
    }

    while let Some(&c) = slice_get(text, j + pat_len_m1) {
        // check the back character of the pattern
        if c == pat_last && slice_subset_eq(&text, j, pat) {
            return Some(j);
        }
        j += skip[c as usize] as usize;
    }
    None
}

#[test]
fn bmh_preprocess() {
    let mut skip = [0; 256];
    let needle = b"gcagagag";
    bmh_skip(needle, &mut skip);
    assert_eq!(skip[b'g' as usize], 2);
    assert_eq!(skip[b'c' as usize], 6);
    assert_eq!(skip[b'a' as usize], 1);
    assert_eq!(skip[b't' as usize], 8);
}

#[test]
fn bmh_find() {
    let text = b"abc";
    assert_eq!(find(text, b"d"), None);
    assert_eq!(find(text, b"c"), Some(2));

    let longer = "longer text and so on";

    // test all windows
    for wsz in 1..17 {
        for window in longer.as_bytes().windows(wsz) {
            let str_find = longer.find(::std::str::from_utf8(window).unwrap());
            assert!(str_find.is_some());
            assert_eq!(find(longer.as_bytes(), window), str_find);
        }
    }

    let pat = b"ger text and so on";
    assert!(pat.len() > 16);
    assert_eq!(Some(3), find(longer.as_bytes(), pat));
}
