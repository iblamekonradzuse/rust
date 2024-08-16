fn main(){
    let s: String = String::from("hello,");

    let mut s1 = s;

    s1.push_str("world");
    println!("{}", s1);
    println!("suc");

}