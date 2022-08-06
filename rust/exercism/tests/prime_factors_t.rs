use exercism::library::prime_factors::*;

#[test]
fn test_prime_factors_case_1() {
    assert_eq!(prime_factors(2), vec![2]);
}

#[test]
fn test_prime_factors_case_2() {
    assert_eq!(prime_factors(60), vec![2, 2, 3, 5]);
}

#[test]
fn test_prime_factors_case_3() {
    assert_eq!(prime_factors(30), vec![2, 3, 5]);
}
