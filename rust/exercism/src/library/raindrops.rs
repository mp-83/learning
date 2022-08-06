/* Link to exercise
 * https://exercism.io/my/solutions/40beb7978320469a9fa68237349f7a88
 */

pub fn convert_drops(drops: i32) -> String {
    let mut ret = String::from("");
    if drops % 3 == 0 {
        ret.push_str("Pling");
    }
    if drops % 5 == 0 {
        ret.push_str("Plang");
    }
    if drops % 7 == 0 {
        ret.push_str("Plong");
    }
    if ret == "" {
        ret = drops.to_string();
    }
    ret
}
