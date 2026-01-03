fn main() {
    let mut person:(&str, i32, i32) = ("Bob", 42, 180);

    println!("{}", person.0);
    println!("{}", person.1);
    println!("{}", person.2);

    person.0 = "Sue";
    println!("{person:?}");

    let (name, age, height) = person;
    println!("{} {} {}", name, age, height);

    let unit:() = ();
}
