fn main() {
    
    let mut value = Some(0);

    while let Some(n) = value {

        value = Some(n + 1);
        
        if n > 5 {
            value = None;
        }

        println!("{n}");
    }
}
