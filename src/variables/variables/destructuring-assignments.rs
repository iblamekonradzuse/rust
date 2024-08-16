fn main() {
    let (x,y);
    (x, ..) = (3, 4);
    [.., y ] = [1, 5];
    assert_eq!([x,y], [3, 5]);
    println!("suc");

}