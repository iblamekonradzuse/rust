fn main(){
    let _v: () = ();

    let v: (i32, i32) = (2,3);
    assert_eq!(_v, implicitly_ret_unit());
    println!("suc");
}

fn implicitly_ret_unit(){
    println!("i return a ()");
}

fn explicitly_ret_unit() -> (){
    println!("i return a ()");
}