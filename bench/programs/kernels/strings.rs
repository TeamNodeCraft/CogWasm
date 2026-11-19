#![no_std]

include!("../prelude.rs");

use alloc::string::String;
use alloc::vec::Vec;

const LINES: usize = 512;

const WORDS: [&str; 20] = [
    "Alpha", "bravo", "Charlie", "delta", "Echo", "foxtrot", "Golf", "hotel", "India", "juliet",
    "Kilo", "lima", "Mike", "november", "Oscar", "papa", "Quebec", "romeo", "Sierra", "tango",
];

fn push_i64(output: &mut String, value: i64) {
    let mut x = value.unsigned_abs();
    if x == 0 {
        output.push('0');
        return;
    }
    let mut digits = [0u8; 20];
    let mut n = 0;
    while x > 0 {
        digits[n] = b'0' + (x % 10) as u8;
        x /= 10;
        n += 1;
    }
    if value < 0 {
        output.push('-');
    }
    while n > 0 {
        n -= 1;
        output.push(digits[n] as char);
    }
}

fn uppercase(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        output.push(bytes[i].to_ascii_uppercase() as char);
        i += 1;
    }
    output
}

fn build_lines(rng: &mut Rng) -> Vec<String> {
    let mut lines: Vec<String> = Vec::with_capacity(LINES);
    let mut i = 0;
    while i < LINES {
        let mut line = String::new();
        let count = 3 + rng.below(6) as usize;
        let mut w = 0;
        while w < count {
            if w > 0 {
                line.push(' ');
            }
            if rng.below(5) == 0 {
                let n = rng.below(2_001) as i64 - 1_000;
                push_i64(&mut line, n);
            } else {
                let word = WORDS[rng.below(WORDS.len() as u64) as usize];
                line.push_str(word);
                if rng.below(8) == 0 {
                    line.push('_');
                    let suffix = rng.below(100) as i64;
                    push_i64(&mut line, suffix);
                }
            }
            w += 1;
        }
        lines.push(line);
        i += 1;
    }
    lines
}

fn body(seed: u64) -> u64 {
    let mut rng = Rng::new(seed);
    let lines = build_lines(&mut rng);

    let mut line_count = 0u64;
    let mut word_count = 0u64;
    let mut char_count = 0u64;
    let mut letter_count = 0u64;
    let mut digit_count = 0u64;
    let mut numeric_count = 0u64;
    let mut numeric_sum = 0i64;
    let mut max_word = 0usize;
    let mut histogram = [0u64; 16];
    let mut alpha_hash = 0x6a09_e667_f3bc_c909u64;
    let mut upper_hash = 0xbb67_ae85_84ca_a73bu64;
    let mut parsed_hash = 0x3c6e_f372_fe94_f82bu64;

    for line in &lines {
        line_count += 1;
        let upper = uppercase(line);

        for token in line.split_whitespace() {
            word_count += 1;
            let bytes = token.as_bytes();
            let len = bytes.len();
            char_count += len as u64;
            if len > max_word {
                max_word = len;
            }
            let bucket = if len < 16 { len } else { 15 };
            histogram[bucket] += 1;

            let mut i = 0;
            while i < len {
                let c = bytes[i];
                if c >= b'0' && c <= b'9' {
                    digit_count += 1;
                } else if (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') {
                    letter_count += 1;
                }
                alpha_hash = alpha_hash.rotate_left(7) ^ c as u64;
                i += 1;
            }

            let first = bytes[0];
            if first == b'-' || (first >= b'0' && first <= b'9') {
                if let Ok(value) = token.parse::<i64>() {
                    numeric_count += 1;
                    numeric_sum = numeric_sum.wrapping_add(value);
                    parsed_hash = parsed_hash.rotate_left(11) ^ value as u64;
                }
            }
        }

        for token in upper.split_whitespace() {
            let bytes = token.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                upper_hash = upper_hash.rotate_left(5) ^ bytes[i] as u64;
                i += 1;
            }
        }
    }

    emit_u64(line_count);
    emit_u64(word_count);
    emit_u64(char_count);
    emit_u64(letter_count);
    emit_u64(digit_count);
    emit_u64(numeric_count);
    emit_i64(numeric_sum);
    emit_u64(max_word as u64);
    let mut i = 0;
    while i < histogram.len() {
        emit_u64(histogram[i]);
        i += 1;
    }
    emit_u64(alpha_hash);
    emit_u64(upper_hash);
    emit_u64(parsed_hash);

    alpha_hash
        ^ upper_hash.rotate_left(13)
        ^ parsed_hash.rotate_left(29)
        ^ numeric_sum as u64
        ^ char_count.rotate_left(43)
}
