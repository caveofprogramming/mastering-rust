/*
    pub enum Option<T> {
        None,
        Some(T),
    }
*/

fn main() {
    let value = Some(123);

    let result = match value {
        Some(number) => number,
        _ => 0,
    };

    println!("result: {result}");
}
