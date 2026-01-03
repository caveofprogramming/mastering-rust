fn main() {
    let text:String = "Hello".to_string();

    let ref1:&String = &text;
    let ref2:&String = &text;

    println!("{text}");
    println!("{}", *ref1);
    println!("{}", *ref2);
    println!("{}", ref1);
}
