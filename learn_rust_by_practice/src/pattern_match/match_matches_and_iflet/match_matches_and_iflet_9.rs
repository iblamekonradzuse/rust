fn main(){
    let age: Option<i32> = Some(30);
    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is a new variable, its value is {}", age),
        _ => ()
    }
}