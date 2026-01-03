fn main() {
    let values = [0, 10, 20, 30, 40];

    for n in values {
        println!("{n}");
    }

    for i in 0..values.len() {
        let v = values[i];
        println!("{i} {v}");
    }

    println!();

    let texts = ["hello", "you"];

    for s in texts {
        println!("{s}");
    }

    println!();
    
    for s in texts {
        println!("{s}");
    }
    
    println!();

    let texts = ["hello".to_string(), "you".to_string()];

    for s in &texts {
        println!("{s}");
    }

    println!();
    
    for s in texts {
        println!("{s}");
    }
}
