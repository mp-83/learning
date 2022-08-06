use exercism::library::raindrops::*;

#[test]
fn test_convert_drops_case_1() {
    assert_eq!(convert_drops(3), "Pling");
}

#[test]
fn test_convert_drops_case_2() {
    assert_eq!(convert_drops(30), "PlingPlang");
}

#[test]
fn test_convert_drops_case_3() {
    assert_eq!(convert_drops(28), "Plong");
}

#[test]
fn test_convert_drops_case_4() {
    assert_eq!(convert_drops(34), "34");
}

#[test]
fn test_convert_drops_case_5() {
    assert_eq!(convert_drops(105), "PlingPlangPlong");
}
