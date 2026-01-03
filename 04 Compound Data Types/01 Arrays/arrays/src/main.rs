fn main() {
    let mut values:[i32; 3] = [6, 8, 4];

    println!("{}", values[0]);
    println!("{}", values[2]);
    
    values[1] = 10;
    println!("{}", values[1]);

    let values:[f64; 10] = [1.23; 10];
    println!("{values:?}");
}