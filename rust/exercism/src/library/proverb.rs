/* Link to exercise
 * https://exercism.io/my/solutions/282e272330c346be97eeb1c143f42254
 */

pub fn format_phrase(word_1: &str, word_2: &str) -> String {
    let s = String::from(&format!(
        "For want of a {} the {} was lost.",
        word_1, word_2
    ));
    s
}

pub fn proverb() {
    let words = [
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let mut i = 0;
    while i < words.len() - 1 {
        println!("{}", format_phrase(words[i], words[i + 1]));
        i += 1;
    }
    println!("And all for the want of a {}.", words[1]);
}
