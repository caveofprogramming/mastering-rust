fn main() {
    let mut value = 123;

    let ref1 = &value;
    println!("{ref1}");

    let ref2:&mut i32 = &mut value;
    println!("{ref2}");

    let ref3:&mut i32 = &mut value;
    println!("{ref3}");
}
