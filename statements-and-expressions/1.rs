fn main(){
    let v: i32 = {
        let mut x: i32 = 1;
        x = x + 2;
        x
    };
    assert_eq!(v,3);

    println!("suc!");
}