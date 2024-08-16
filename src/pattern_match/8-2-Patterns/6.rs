fn main(){
    let mut v: String = String::from("hello,");
    let r: &mut String = &mut v;

    match r {
        value => value.push_str("world!")
   }
   println!("{}", v);
   println!("suc");
}