fn main(){
    let mut s: String = String::from("hello,");

    let r1: &mut String = &mut s;
    r1.push_str("world");
    println!("{}", r1);

    let r2: &mut String = &mut s;
    r2.push_str("!");

    println!("{}",r2);
}
//we are not using r1 after r1.pust_str so commenting the line that we use r1 again willl make the code compile 
//we can also print out r2, it will compile anyway
