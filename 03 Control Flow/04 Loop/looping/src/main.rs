fn main() {

    let mut count = 0;

    let result = loop {
        println!("Hello {count}");
        count += 1;

        if count == 5 {
            break count;
        }
    };

    println!("{result}");
}
