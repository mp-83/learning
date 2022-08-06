use exercism::library::square_diff::*;

#[test]
fn test_process_array_case_1() {
    assert_eq!(process_array(&vec![1, 2, 3]), (6, 14));
}

#[test]
fn test_process_array_case_2() {
    assert_eq!(process_array(&vec![1]), (1, 1));
}

#[test]
fn test_square_diff_case_1() {
    assert_eq!(square_diff(3), 22);
}

#[test]
fn test_square_diff_case_2() {
    assert_eq!(square_diff(10), 2640);
}

#[test]
fn test_generate_vector_case_1() {
    assert_eq!(generate_vector(5), [1, 2, 3, 4, 5]);
}

#[test]
fn test_generate_vector_case_2() {
    assert_eq!(generate_vector(0), []);
}
