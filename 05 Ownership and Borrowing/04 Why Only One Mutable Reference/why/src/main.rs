fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let mut p = Point { x: 10, y: 20 };

        let mut_ref = &mut p;

        p = Point { x: 50, y: 60 };

        println!("{:?}", mut_ref); 
    }
}
