fn main() {

    fn run() {
        println!("Hello");
    };

    test(run)
}

fn test<F>(run: F)
where
    F: Fn(),
{
    run();
}
