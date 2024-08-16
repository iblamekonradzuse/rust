fn main() {
    let s: String = String::from("hello, 世界");
    let slice1: &str = &s[..1];
    assert_eq!(slice1, "h");

    let slice2: &str = &s[10..13];
    assert_eq!(slice2, "界");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("suc");
}
