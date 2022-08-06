use exercism::library::nth_prime::*;

#[test]
fn test_nth_prime_num_case_1() {
    assert_eq!(nth_prime_num(2), 5);
}

#[test]
fn test_nth_prime_num_case_2() {
    assert_eq!(nth_prime_num(19), 71);
}
