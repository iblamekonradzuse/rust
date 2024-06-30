fn main(){
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    //Integer subtraction
    assert!(1i32 -2i32 == -1i32);
    assert!(1i8 -2i8 == -1);

    assert!(3* 50 == 150);
    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24 % 5 == 4);
    //short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    //bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1<< 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

}