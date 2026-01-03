fn main() {
    let value = 2;

    match value {
        0 => println!("zero"),
        1|2 => println!("one or two"),
        3..10 => println!("in range 3..10"),
        _ => println!("something else"),
    }
}
