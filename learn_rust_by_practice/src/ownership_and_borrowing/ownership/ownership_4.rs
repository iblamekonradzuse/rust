fn main(){
    let s: String = String::from("hello, world");

    print_str(s.clone());

    println!("{}" , s);
}

fn print_str(s: String) {
    println!("{}", s)
}