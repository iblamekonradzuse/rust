fn main(){
    let mut s = String::from("hello,");

    borrow_object(&mut s);

    println!("suc");
}

fn borrow_object(s: &mut String){}