/*

pub enum Result<T, E> {
    Ok(T),  
    Err(E),
}

*/

fn main() {
    let ok:Result<i32, &str> = Result::Ok(123);
    let error:Result<i32, &str> = Result::Err("Something went wrong!");

    let result = error;
    println!("{result:?}");

    match result {
        Result::Ok(x) => println!("{x}"),
        Result::Err(s) => println!("{s}"),
    }
}
