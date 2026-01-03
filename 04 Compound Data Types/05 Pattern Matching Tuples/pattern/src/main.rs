fn main() {
    let person: (&str, i32, i32, i32) = ("Bob", 40_000, 42, 180);

    let (name, salary, age, height) = person;
    println!("{name} {salary} {age} {height}");

    let (name, _, age, _) = person;
    println!("{name} {age}");

    let (name, .., height) = person;
    println!("{name} {height}");

    let whole @ (name, .., height @ 160..=180) = person else {
        unreachable!("oh no");
    };
    println!("{name} {height}, {whole:?}");

    let people = [("Bob", 42), ("Sue", 40)];

    for (name, age) in people {
        println!("{name} {age}");
    }
}
