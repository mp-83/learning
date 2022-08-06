use exercism::library::matching_brackets::*;

#[test]
fn test_matching_brackets_case_1() {
    assert_eq!(are_balanced("{[]"), false);
}

#[test]
fn test_matching_brackets_case_2() {
    assert_eq!(are_balanced("[]["), false);
}

#[test]
fn test_matching_brackets_case_3() {
    assert_eq!(are_balanced("[]"), true);
}

#[test]
fn test_matching_brackets_case_4() {
    assert_eq!(are_balanced("{{}}("), false);
}

#[test]
fn test_matching_brackets_case_5() {
    assert_eq!(are_balanced("[][]["), false);
}

#[test]
fn test_matching_brackets_case_6() {
    assert_eq!(are_balanced("{[()]}"), true);
}

#[test]
fn test_matching_brackets_case_7() {
    assert_eq!(are_balanced("}{}"), false);
}

#[test]
fn test_matching_brackets_case_8() {
    assert_eq!(are_balanced("][]"), false);
}
