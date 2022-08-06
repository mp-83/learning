/* Link to exercise
 * https://exercism.io/my/solutions/a65aeeaccfe441e2961d7e3da892120e
 */

pub fn reverse_string(s: &str) -> String {
    let n = s.len();
    let mut new_vec = Vec::new();
    let sbytes = s.as_bytes();
    for i in (0..n).rev() {
        new_vec.push(sbytes[i] as char);
    }
    new_vec.iter().collect()
}
