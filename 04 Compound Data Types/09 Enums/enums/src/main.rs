fn main() {

    #[derive(Debug, Clone)]
    #[repr(u8)]
    enum Pet {
        Dog = 10,
        Cat = 20,
        Mouse = 30,
    }

    let pet:Pet = Pet::Mouse;

    println!("{pet:?}");
    println!("{}", pet.clone() as u8);

    match pet {
        Pet::Dog => println!("Dog detected"),
        Pet::Cat => println!("Cat detected"),
        _ => println!("No match"),
    }

}
