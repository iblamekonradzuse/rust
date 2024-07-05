fn main() {
    let mut s: String = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");
    println!("suc");
}

