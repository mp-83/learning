/* Link to exercise
 * https://exercism.io/my/solutions/6af083db8c1f4e0e8923f4dbcda0922f
 */

pub fn num_to_vec(n: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut i = n;
    loop {
        vec.push(i % 10);
        i /= 10;
        if i == 0 {
            break;
        }
    }
    vec.reverse();
    vec
}

pub fn is_armstrong_number(n: i32) -> bool {
    // converting the number to a vector was the
    // easiest way for me right now to iterate
    // over its digits. Probably there is another
    // way to achieve the same results using its
    // bytes representation
    let n2v = num_to_vec(n);
    let ndigits = n2v.len() as u32;
    let mut sum = 0;
    for i in n2v.iter() {
        sum += i.pow(ndigits);
    }
    sum == n
}
