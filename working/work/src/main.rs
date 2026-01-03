fn main() {

    /* 
     *  _: match and discard
     * ..: match multiple
     * (name, n @ age, h @ height): variable binding for match guards
     * 
     */
    let person = ("Bob", 42, 180);

    let (name, ..) = person;
    let (.., height) = person;

    
    println!("{name}");
    println!("{height}");
    
    let whole @ (_, 42, n @ name) = person else {
        unreachable!();
    };

    match person {
        whole @ (name, n @ age, h @ height) if n > 20 && h < 200 => println!("Matched {name} {age} {height}, {whole}"),
        _ => println!("No match"),
    }

    println!("{name}");
}