fn main(){
    let mut s: String = String::from ("hello,");

    let p: &mut String = &mut s;

    p.push_str("world");
    println!("{}", p);
    println!("suc");
    
}