fn main(){
    let mut s: String = String::from("hello,");

    push_str(&mut s);
    println!("suc"); 
    println!("{}", s);
}

fn push_str(s: &mut String){
    s.push_str("world")
}