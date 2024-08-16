fn main(){
    let v: i32 = {
        let x = 3;
        x 
    };

    assert!(v == 3);
    println!("suc!");
}