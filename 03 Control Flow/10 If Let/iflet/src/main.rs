fn main() {
    let number: Option<i32> = None; // Some(7);

    let result = if let Some(n) = number {
        n
    }
    else {
        0
    };

    println!("{result}");
}
