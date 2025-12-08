/*
 * The Rust Reference: Destructors
 * Each variable or temporary is associated
 * to a drop scope. When control flow leaves a
 * drop scope all variables associated to that scope
 * are dropped in reverse order of declaration
 * (for variables) or creation (for temporaries).
 */

struct Test(&'static str);

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let t = Test("one");

    {
        let t = Test("two");
    }

    let t = Test("three");
    drop(t);

    let t = Test("four");
}
