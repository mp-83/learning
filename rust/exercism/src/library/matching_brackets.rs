/* Link to exercise
 * https://exercism.io/my/solutions/4551edf6b94c4883962444ef1496b074
 */

pub fn are_balanced(string: &str) -> bool {
    let mut square = 0;
    let mut curly = 0;
    let mut round = 0;
    let s = String::from(string);
    for ch in s.chars() {
        if ch == '[' {
            square += 1;
        } else if ch == ']' {
            square -= 1;
        } else if ch == '{' {
            curly += 1;
        } else if ch == '}' {
            curly -= 1;
        } else if ch == '(' {
            round += 1;
        } else if ch == ')' {
            round -= 1;
        }
    }
    (square == 0) & (curly == 0) & (round == 0)
}
