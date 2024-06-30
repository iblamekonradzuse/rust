struct Person{
    name: String,
    age: u8,
}

fn main(){
    println!("suc");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}