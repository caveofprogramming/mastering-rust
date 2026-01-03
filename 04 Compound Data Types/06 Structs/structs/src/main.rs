fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let mut person1 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    person1.name = String::from("Aubrey");

    let person2 = Person {
        name: String::from("Sue"),
        age: 31,
    };

    println!("{}, {}", person1.name, person1.age);
    println!("{}, {}", person2.name, person2.age);

    println!("{:?}", person1);
}
