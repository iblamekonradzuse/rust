fn main(){
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("suc");
            panic!("we have no value for 'false' but we can panic! ");
        }
    };
    println!("failed!");
}