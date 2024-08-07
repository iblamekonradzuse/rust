fn main() {
    let mut s: String = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("helllo");
        println!("{}", s.capacity());
    }

    println!("suc");
    println!("{}", s);
}
