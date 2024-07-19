#[derive(Debug, PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = Point { x: 3, y: 4 };

    println!("p1 < p2: {}", p1 < p2); // true
    println!("p2 > p3: {}", p2 > p3); // false
}
