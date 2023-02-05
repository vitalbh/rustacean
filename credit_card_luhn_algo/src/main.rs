/**
Ignore all spaces. Reject number with less than two digits.

Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.

After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.

Sum all the undoubled and doubled digits.

The credit card number is valid if the sum ends with 0.
 **/

pub fn luhn(cc_number: &str) -> bool {
    let mut visited = 0;
    let mut sum = 0;

    for (i, num) in cc_number.
            chars().
            filter(| &ch| ch !=' ').
            rev().
            enumerate() {
                match num.to_digit(10) {
                    Some(digit) => {
                        sum += if i % 2 == 1 {
                            let d = digit * 2;
                            d / 10 + d % 10
                        } else {
                            digit
                        };
                        visited +=1;
                    }
                    None => {
                        return false
                    }
                }
    }
    if visited < 2 {
        return false
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

fn print_is_valid(cc_number: &str) {
    match luhn(cc_number) {
        true => println!("the credit card {} {} valid",cc_number, "is"),
        false => println!("the credit card {} {} valid",cc_number, "isn't")
    }
}
#[allow(dead_code)]
fn main() {
    print_is_valid("4539 3195 0343 6467");
    print_is_valid("8273 1232 7352 0569");
    print_is_valid("8273 1232 72 09");

}