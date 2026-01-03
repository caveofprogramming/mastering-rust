fn main() {

    let mut count = 0;

    let result = 'running: loop {
        println!("loop {count}");

        if count > 5 {
            break 'running 123;
        }

        count += 1;
    };

    println!("result: {result}");
}

