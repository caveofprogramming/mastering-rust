fn main() {
   
    let value = 10;

    match value {
        n @ 0..=20 if n % 2 == 0 => println!("match: {n}"),
        n => println!("no match: {n}"),
    }
}
