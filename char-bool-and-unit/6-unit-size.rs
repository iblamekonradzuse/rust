use std::mem::size_of_val;
fn main (){
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("suc!");
}