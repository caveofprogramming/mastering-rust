fn main() {
    let bob:(&str, i32) = ("Bob", 42);
    println!("{} {}", bob.0, bob.1);

    #[derive(Debug)]
    struct Bob {
        name: String,
        age: i32,
    }

    let bob = Bob {
        name: "Bob".to_string(),
        age: 42,
    };

    println!("{} {}", bob.name, bob.age);

    #[derive(Debug)]
    struct Person(String, i32);

    let sue = Person("Sue".to_string(), 35);
    println!("{} {}", sue.0, sue.1);
}
