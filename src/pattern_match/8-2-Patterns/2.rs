struct Point {
    x: i32,
    y: i32,
}
fn main(){
    let p = Point { x: 3, y:30 };

    match p {
        Point {x, y:0} => println!("on the x axis at {}", x),
        Point {x:0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point {x, y } => println! ("on neither axis: ({}, {})", x, y),
    }
}