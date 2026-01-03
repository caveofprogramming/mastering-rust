fn main() {
    struct Point(i32, i32);

    enum Shape {
        Point(Point),
        Circle { radius: i32, centre: Point },
    }

    let point = Shape::Point(Point(3, 4));
    let circle = Shape::Circle {
        radius: 7,
        centre: Point(8, 9),
    };

    let s = circle;

    match s {
        Shape::Point(Point(x, y)) => println!("{x}, {y}"),
        Shape::Circle {
            radius: r,
            centre: Point(x, y),
        } => println!("{r}, {x}, {y}"),
    }
}
