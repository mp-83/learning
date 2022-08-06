/* Link to exercise
 * https://exercism.io/my/solutions/aa981078f2d140dba71a22e0aa5e26ca
 */

pub fn sum_multiples(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
