// const DEFAULT_POSITION: char = 'a';

// pub const fn chars_sub(chr1: char, chr2: char) -> i32 {
//     (chr1 as i32).saturating_sub(chr2 as i32)
//     }

// pub const fn def_chars_sub(chr: char) -> usize {
//     chars_sub(chr, DEFAULT_POSITION) as usize
//     }

pub fn distance(dx: i32, dy: i32) -> f64 {
    ((dx * dx + dy * dy) as f64).sqrt()
    }