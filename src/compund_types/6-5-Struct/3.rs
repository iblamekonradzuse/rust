struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("suc");
}

fn check_color(p: Point){
    let Point(x, _, z) = p;
    assert_eq!(x,0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}