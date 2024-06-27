fn main(){
    let arr: [i32; 3] = [1, 2 ,3];
    let s1: &[i32] = &arr[0..2]; // &[1,2]

    let s2: &str = "hello world";
    println!("suc");
}