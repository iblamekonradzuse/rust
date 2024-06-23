fn main(){
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;

    println!("the persons age is {}", age);
    println!("the persons name is {}", name);

    println!("the persons age from persons struct is {}", person.age); 
}