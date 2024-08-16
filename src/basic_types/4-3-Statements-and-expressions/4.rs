fn main(){
    let s: i32 = sum(1,2);
    assert_eq!(s, 3);

    println!("suc!");

} 

fn sum(x: i32, y: i32) -> i32 {
    x + y
}