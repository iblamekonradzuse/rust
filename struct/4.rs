struct Person {
    name: String,
    age: u8,
}

fn main(){
    let age: u8 = 18;
    let mut p: Person = Person {
        name: String::from("sunface"),
        age,
    };

    p.age = 30;

    p.name = String::from("sunfei");
    println!("{},{}",p.name, p.age);
    println!("suc");
}