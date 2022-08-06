/* Link to exercise
 * https://exercism.io/my/solutions/3d518068710647bebb62139f8a0913e6
 */

pub fn bob_answers(string: &str) -> &'static str {
    let mut s = string;
    let response;

    if string != "How are you?" && string.ends_with("?") {
        s = "question"
    }
    match s {
        "YELL AT HIM" => response = "Whoa, chill out!",
        "How are you?" => response = "Sure",
        "question" => response = "Calm down, I know what I'm doing!",
        _ => response = "Whatever.",
    }

    response
}
