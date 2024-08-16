fn main() {
    let s: String = String::from("hello, world!");

    let slice: &str = &s;

    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("suc");
}
