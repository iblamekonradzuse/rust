fn main() {
    let s: &str = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}

//to.sting() &str to string
