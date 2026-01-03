
fn main() {
    enum Shape {
        Null,
        Point(i32, i32),
        Circle { radius: i32, x: i32, y: i32 },
    }

    let s1 = Shape::Null;
    let s2 = Shape::Point(5, 6);
    let s3 = Shape::Circle {
        radius: 5,
        x: 6,
        y: 7,
    };

    let s = s3;

    match s {
        Shape::Null => println!("Null"),
        Shape::Point(x, y) => println!("Point {x} {y}"),
        Shape::Circle { radius:5, x, y } => println!("Circle radius:5 {x} {y}"),
        Shape::Circle { radius, x, y } => println!("Circle {radius} {x} {y}"),
    }
}
