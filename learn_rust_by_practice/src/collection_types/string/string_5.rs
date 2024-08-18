fn main() {
    let mut s: String = String::new();
    s.push_str("hello");

    let v: Vec<u8> = vec![104, 101, 108, 108, 111];

    let s1: String = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("suc");
}
