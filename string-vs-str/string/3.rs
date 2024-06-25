fn main(){
    let mut s: String = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    println!("suc");
}