use exercism::library::leap::*;

#[test]
fn test_is_divisible_by_case_1() {
    assert_eq!(is_divisible_by(2, 4), false);
}

#[test]
fn test_is_divisible_by_case_2() {
    assert_eq!(is_divisible_by(16, 4), true);
}

#[test]
fn test_is_divisible_by_case_3() {
    assert_eq!(is_divisible_by(1900, 100), true);
}

#[test]
fn test_is_divisible_by_case_4() {
    assert_eq!(is_divisible_by(2000, 400), true);
}

#[test]
fn test_is_divisible_by_case_5() {
    assert_eq!(is_divisible_by(1700, 400), false);
}

#[test]
fn test_is_leap_case_1() {
    assert_eq!(is_leap(1600), true);
}

#[test]
fn test_is_leap_case_2() {
    assert_eq!(is_leap(1700), false);
}

#[test]
fn test_is_leap_case_3() {
    assert_eq!(is_leap(2000), true);
}

#[test]
fn test_is_leap_case_4() {
    assert_eq!(is_leap(2008), true);
}

#[test]
fn test_is_leap_case_5() {
    assert_eq!(is_leap(1997), false);
}
