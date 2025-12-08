fn main() {
    let value1 = 8;
    let value2 = value1;
    println!("{:p}", &value1);
    println!("{:p}", &value2);

    let text1:&str = "Hello";
    let text2:&str = text1;
    println!("{:p}", text1);
    println!("{:p}", text2);

    let text1 = String::from("Hello");
    let text2 = text1;
    //println!("{}", text1);
    println!("{}", text2);
}

