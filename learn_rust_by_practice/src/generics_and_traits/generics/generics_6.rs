struct Point <T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U>{
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main(){
    let p1: Point<i32, i32> = Point {x: 5, y: 10};
    let p2: Point<&str, char> = Point { x:"Hello", y: 'a'};
    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'a');

    println!("suc");
}