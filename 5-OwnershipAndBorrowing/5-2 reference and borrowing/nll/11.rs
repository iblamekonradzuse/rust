fn main() {
    let mut s: String = String::from("hello,");

    let r1 = &mut s;
    // the method that works:
    println!("{}", r1);

    let r2 = &mut s;
    // add one line to make an compiler error
    //println!("{}, {}", r1,r2);

    // the method that works:
    println!("{}", r2);
}

