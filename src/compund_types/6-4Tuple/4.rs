fn main(){
    let tup: (i32, f64, &str) = (1, 6.4, "hello");
    let (x, z, y) = tup;

    assert_eq!(x,1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    println!("suc");
}