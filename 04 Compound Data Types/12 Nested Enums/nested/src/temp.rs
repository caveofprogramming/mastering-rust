fn main() {

    struct Point(i32, i32);

    enum Shape {
        Point(Point),
        Circle{ radius: i32, centre: Point },
    }

    let shape1 = Shape::Circle{ 
        radius: 5,
        centre: Point(2, 3), 
    };

    let shape2 = Shape::Point(Point(5, 6));

    let s = shape2;

    match s {
        Shape::Point(Point(x, y)) => println!("Point: {x},{y}"),
        Shape::Circle{radius:r, centre: Point(x, y)}  => println!("Circle: {x},{y},{r}"),
    }

    match s {
        Shape::Point(Point(x, _)) => println!("Point: {x}"),
        Shape::Circle{radius:r, ..}  => println!("Circle: {r}"),
    }
}
