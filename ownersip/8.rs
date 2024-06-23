fn main(){
    let t:(String, String) = (String::from("hello"), String::from("world"));

    let _s = t.0;

    println!("{:?}", t.1);
}