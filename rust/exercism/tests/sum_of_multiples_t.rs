use exercism::library::sum_of_multiples::*;

#[test]
fn test_sum_multiples_case_1() {
    assert_eq!(sum_multiples(3), 0);
}

#[test]
fn test_sum_multiples_case_2() {
    assert_eq!(sum_multiples(20), 78);
}
