fn main(){
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2 = &s;

  //  println!("{}, {}", r1 , r2);
    println!("s: {}", s);
    println!("{}", r1);
    println!("{} ",r2);
}