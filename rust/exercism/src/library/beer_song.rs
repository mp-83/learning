/* Link to exercise
 * https://exercism.io/my/solutions/ff81b2b539dc4b38baca199605686958
 */

pub fn format_verse(n: i32) -> String {
    let mut s = String::from("");
    if n > 2 {
        s.push_str(&format!(
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.",
            n,
            n,
            n - 1
        ));
    } else if n == 1 {
        s.push_str(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.",
        );
    } else {
        s.push_str(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.",
        );
    }
    s
}

pub fn print_song() {
    let mut n = 99;
    while n > 0 {
        println!("{}", format_verse(n));
        n -= 1;
    }
}
