fn main(){
    for n in 1..100 {
        if n == 100 {
            panic!("never let this run")
        }
    }
    println!("suc");
}