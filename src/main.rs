// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn digits(idx: usize, c: char) -> Vec<i32> {
    let dig: i32 = c.to_digit(10).unwrap_or(0).try_into().unwrap_or(0); // no error checking

    if idx % 2 != 0 {
        let r = dig * 2 % 10;
        let l = (dig * 2 - r) / 10;
        vec![l, r]
    } else {
        vec![dig]
    }
}
pub fn luhn(cc_number: &str) -> bool {
    let ccn = cc_number.replace(" ", "");
    if ccn.len() < 2 || !ccn.chars().all(|c| c.is_numeric()) {
        return false;
    }
    ccn.chars()
        .rev()
        .enumerate()
        .flat_map(|(idx, c)| digits(idx, c))
        .sum::<i32>()
        % 10
        == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
