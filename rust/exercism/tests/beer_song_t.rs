use exercism::library::beer_song::*;

#[test]
fn test_format_verse_case_1() {
    assert_eq!(format_verse(12), "12 bottles of beer on the wall, 12 bottles of beer.\nTake one down and pass it around, 11 bottles of beer on the wall.");
}

#[test]
fn test_format_verse_case_2() {
    assert_eq!(format_verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.");
}

#[test]
fn test_format_verse_case_3() {
    assert_eq!(format_verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.");
}
