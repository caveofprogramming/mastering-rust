fn main() {

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: i32,
    }

    let person1 = Person {
        name: "Bob".to_string(),
        age: 42,
    };

    let person2 = person1.clone();

    println!("{person1:?}");
    println!("{person2:?}");
}
