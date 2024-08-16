fn main(){
    let s: String = String::from("hello");
    let slice1: &str = &s[0..2];
    let slice2: &str = &s[..2];

    assert_eq!(slice1, slice2);
    println!("suc");
}