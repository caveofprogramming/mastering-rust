fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 5, y: 6 };

    let Point { x, y } = p;
    println!("{x},{y}");

    let Point { x:a, y:b } = p;
    println!("{a},{b}");

    let value = (1, 2, p, Point{ x:10, y:20});
   
    let (r1, r2, Point{x:x1, y:y1}, Point{x:x2, y:y2}) = value;
    println!("{r1}:{r2}, {x1}:{y1}, {x2}:{y2}");
}
