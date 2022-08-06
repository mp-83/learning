/* Link to exercise
 * https://exercism.io/my/solutions/10af38096eec42ef86079077dacbc1cc
 */

pub fn process_array(numbers: &Vec<i32>) -> (i32, i32) {
    let mut sum: i32 = 0;
    let mut square: i32 = 0;
    for num in numbers.iter() {
        sum += num;
        square += num * num;
    }
    (sum, square)
}

pub fn generate_vector(len: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    for i in 0..len {
        vec.push(i + 1);
    }
    vec
}

pub fn square_diff(n: i32) -> i32 {
    let vec = generate_vector(n);
    let (sum, square) = process_array(&vec);
    sum * sum - square
}
