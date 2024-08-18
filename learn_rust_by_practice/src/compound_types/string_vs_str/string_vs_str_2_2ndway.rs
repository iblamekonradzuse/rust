fn main(){
    let s: &str = "hello world";
    greetings(s)
}

fn greetings(s: &str){
    println!("{}",s)
}
