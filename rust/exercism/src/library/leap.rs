/* Link to exercise
 * https://exercism.io/my/solutions/c15a553f36254622b813cfbfeb3da077
 */

pub fn is_leap(year: i32) -> bool {
    let mut leap = false;
    if is_divisible_by(year, 4) {
        if is_divisible_by(year, 100) {
            if is_divisible_by(year, 400) {
                leap = true;
            }
        } else {
            leap = true;
        }
    }
    leap
}

pub fn is_divisible_by(num: i32, div: i32) -> bool {
    num % div == 0
}
