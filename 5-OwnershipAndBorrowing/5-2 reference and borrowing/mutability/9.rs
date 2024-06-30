// borrow a mutable object as immutable is possible
fn main(){
    let mut s = String::from("hello,");

    borrow_object(&s);

    s.push_str("world");
    println!("suc");
}

fn borrow_object(s: &String){}

