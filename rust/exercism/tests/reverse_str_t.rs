use exercism::library::reverse_str::*;

#[test]
fn test_reverse_string_case_1() {
    assert_eq!(reverse_string("dmx"), "xmd".to_string());
}

#[test]
fn test_reverse_string_case_2() {
    assert_eq!(reverse_string("u"), "u".to_string());
}

#[test]
fn test_reverse_string_case_3() {
    assert_eq!(reverse_string("str"), "rts".to_string());
}
