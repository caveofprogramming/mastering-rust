fn main() {
    let value:Option<i32> = Some(7);
    println!("{value:?}");
    println!("{}", value.unwrap());
    
    let value:Option<i32> = None;
    println!("{value:?}");
    println!("{}", value.unwrap_or(0));
    println!("{}", value.expect("no value set"));
}
