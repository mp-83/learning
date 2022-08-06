use exercism::library::armstrong_number::*;

#[test]
fn test_num_to_vec_case_1() {
    assert_eq!(num_to_vec(9), vec![9]);
}

#[test]
fn test_num_to_vec_case_2() {
    assert_eq!(num_to_vec(153), vec![1, 5, 3]);
}

#[test]
fn test_num_to_vec_case_3() {
    assert_eq!(num_to_vec(154), vec![1, 5, 4]);
}

#[test]
fn test_is_armstrong_number_case_1() {
    assert_eq!(is_armstrong_number(154), false);
}

#[test]
fn test_is_armstrong_number_case_2() {
    assert_eq!(is_armstrong_number(153), true);
}

#[test]
fn test_is_armstrong_number_case_3() {
    assert_eq!(is_armstrong_number(9), true);
}

#[test]
fn test_is_armstrong_number_case_4() {
    assert_eq!(is_armstrong_number(11), false);
}

#[test]
fn test_is_armstrong_number_case_5() {
    assert_eq!(is_armstrong_number(370), true);
}

#[test]
fn test_is_armstrong_number_case_6() {
    assert_eq!(is_armstrong_number(371), true);
}

#[test]
fn test_is_armstrong_number_case_7() {
    assert_eq!(is_armstrong_number(372), false);
}
