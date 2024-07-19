use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Implementing Add for Point
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    // Using the add method directly
    let p3 = p1.add(p2);

    println!("{:?}", p3); // Point { x: 4, y: 6 }
}
