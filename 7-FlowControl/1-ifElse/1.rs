fn main(){
    let n: i32 = -3;

    if n < 0{
        println!("{} is negative", n);    
    } else if n > 0 {
        println!("{} is positive ", n);
    } else {
        println!("{} is zero", n);
    }
}