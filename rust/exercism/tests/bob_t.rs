use exercism::library::bob::*;

#[test]
fn test_bob_answers_case_1() {
    assert_eq!(bob_answers("How are you?"), "Sure");
}

#[test]
fn test_bob_answers_case_2() {
    assert_eq!(bob_answers("YELL AT HIM"), "Whoa, chill out!");
}

#[test]
fn test_bob_answers_case_3() {
    assert_eq!(
        bob_answers("WHAT ARE YOU DOING?"),
        "Calm down, I know what I'm doing!"
    );
}

#[test]
fn test_bob_answers_case_4() {
    assert_eq!(bob_answers("I go to bed"), "Whatever.");
}
