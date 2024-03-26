
// 1. Luhn algorithm
fn sum_digits(x: i64) -> i64 {
    (x % 10) + (0..)
        .scan(x, |num, _| {
            *num /= 10;
            Some(*num)
        })
        .take_while(|num| *num > 0)
        .map(|num| num % 10)
        .sum::<i64>()
    // while i != 0 { sum += i % 10; i /= 10; };
}

pub fn luhn(cc_number: &str) -> bool {
    let mut copy_numbers = String::from(cc_number.replace(" ", ""));
    copy_numbers = copy_numbers.chars().filter(|char| char.is_digit(10)).collect();
    if copy_numbers.len() < 2 {return false};
    let iterator_copy = copy_numbers.clone();
    for (i, c) in iterator_copy[..iterator_copy.len()-1].char_indices().rev().step_by(2) {
        let num = c.to_digit(10).unwrap() * 2;
        let sum = sum_digits(num.into()).to_string();
        copy_numbers.replace_range(i..i+1, &sum);
    }
    let final_num = sum_digits(copy_numbers.parse::<i64>().unwrap()).to_string();
    if final_num.chars().last().unwrap() != '0' {return false}
    return true;
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
    // (8)2(12)3 (18)8(4)6 (8)046 9299
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

// 2. Strings and Iterators

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_v: Vec<&str> = prefix.split("/").collect();
    let request_v: Vec<&str> = request_path.split("/").collect();
    for (pos, e) in prefix_v.iter().enumerate() {
        if request_v.get(pos) != Option::from(e) && *e != "*" {
            return false;
        }
    }
    return true;
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {}
