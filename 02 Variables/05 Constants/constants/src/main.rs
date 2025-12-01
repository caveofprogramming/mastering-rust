fn main() {
    const PI:f64 = 3.141592;

    println!("{PI}");

    let value1 = 1.23;
    const VALUE2:f64 = 2.34;
    let value3 = 2.45;
    let value4 = 5.67;

    println!("{:p}", &value1);
    println!("{:p}", &VALUE2);
    println!("{:p}", &value3);
    println!("{:p}", &value4);
}
