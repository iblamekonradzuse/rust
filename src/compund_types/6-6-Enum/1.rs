enum Number{
    Zero,
    One,
    Two
}

enum Number1{
    Zero = 0, //0
    One, //1
    Two, //2
}

enum Number2{
    Zero = 5,
    One = 1,
    Two, //7
}

enum Number3{
    Zero = 5,
    One,
    Two,
}
fn main(){
    assert_eq!(Number::One as u8, Number1:: One as u8);
    assert_eq!(Number::One as u8, Number2::One as u8);
    println!("suc");
    
    println!("{}", Number1::One as u8);
    println!("{}", Number2::One as u8);
    println!("{}", Number3::One as u8);
}