use exercism::library::proverb::*;

#[test]
fn test_proverb_case_1() {
    assert_eq!(
        format_phrase("nail", "shoe"),
        "For want of a nail the shoe was lost."
    );
}
