fn main() {
    for i in 0..4 {
        print!("{i} ");
    }

    println!();

    for i in 0..=4 {
        print!("{i} ");
    }

    println!();
    
    for i in (2..6).rev() {
        print!("{i} ");
    }

    println!();
    
    for i in (10..=20).rev().step_by(2) {
        print!("{i} ");
    }
    
    println!();

    let iterator = 100..110;

    for j in iterator.rev() {
        print!("{j} ");
    }
    
}
