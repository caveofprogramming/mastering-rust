
fn main() {
    for i in 1..=5 {
        print!("{i} ");
    }

    println!();
    
    let range:std::ops::Range<i32> = 10..15;
    
    for i in range {
        print!("{i} ");
    }

    println!();

    for i in (3..7).rev() {
        print!("{i} ");
    }

    println!();

    for i in (10..=20).rev().step_by(2) {
        print!("{i} ");
    }
}
