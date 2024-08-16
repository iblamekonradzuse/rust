fn main(){
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets{
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
    }

    println!("suc");
}