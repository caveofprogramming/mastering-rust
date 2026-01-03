fn main() {
    let mut values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut s1:&[i32] = &values[3..=5];
    s1 = &values[0..5];

    println!("{s1:?}");

    let s2 = &mut values[4..7];
    s2[0] = 123;
    println!("{s2:?}");
    println!("{values:?}");
}
