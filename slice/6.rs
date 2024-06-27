fn main(){
    let mut s: String = String::from("hello world");

    let word: &str = first_word(&s);
    
    println!(" the first word is:{}",word);

    s.clear();
}

fn first_word(s: &String) -> &str{
    &s[..1]
}