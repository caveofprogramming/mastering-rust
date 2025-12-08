fn main() {
    
    let mut count = 5;

    let value = while count > 0 {
        count -= 1;
        
        if count == 3 {
            continue;
        }

        println!("loop {count}");
    };
}
