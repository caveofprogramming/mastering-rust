fn main() {
    let greeting1:&str = "Hello";
    println!("{greeting1}");
    
    let greeting2:String = String::from("Hello");
    println!("{greeting2}");

    let greeting3:String = "Hello".to_string();
    println!("{greeting3}");
}
