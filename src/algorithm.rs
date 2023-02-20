pub fn luhn(cc_number: &str) -> bool {
    let chars: Vec<_> = cc_number
        .trim()
        .replace(" ", "")
        .chars()
        .rev()
        .collect();

    if chars.len() < 2 {
        return false;
    }

    let mut sum = 0;

    for (idx, el) in chars.iter().enumerate() {
        match el.to_digit(10) {
            Some(n) => {
                sum += if idx % 2 != 0 {
                    (n*2).to_string()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .sum::<u32>()
                } else {
                    n
                }
            },
            None => {
                return false;
            }
        }
    }

    sum % 10 == 0
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