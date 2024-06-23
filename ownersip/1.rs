fn main(){
    let x: String = String::from("hello, world");

    let y: String = x.clone();

    println!("{}, {}", x , y);
}