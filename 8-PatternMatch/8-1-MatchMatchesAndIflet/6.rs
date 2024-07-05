fn main(){
    let o: Option<i32> = Some(7);
    
    if let Some(i) = o {
    println!("This is a really long string and `{:?}`",i);
    }
 }
    
