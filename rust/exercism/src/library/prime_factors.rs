/* Link to exercise
 * https://exercism.io/my/solutions/5541f8704b504ae1babcfc0cb4f52e96
 */

pub fn prime_factors(n: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut x = n;
    // 1 is not a prime number. this is why
    // i is initialised to 2
    let mut i = 2;
    while i <= n {
        if x % i == 0 {
            x = x / i;
            vec.push(i);
        } else {
            i += 1;
        }
    }
    vec
}
