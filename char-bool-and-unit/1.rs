use std::mem::size_of_val;


fn main (){
    let c1: char = 'a'; // 4 bytes
    assert_eq!(size_of_val(&c1),4);

    let c2: char = '®';
    assert_eq!(size_of_val(&c2),4);

 println!("suc!");
}