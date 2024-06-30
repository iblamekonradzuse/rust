fn main(){
    #[derive(Debug)]
    struct Person{
        name: String,
        age:Box<u8>,
    }

    let person = Person {
        name: String::from("testperson"),
        age: Box::new(20),
    };

    let Person {name, ref age} = person;

    println!("the persons age is {}", age);

    println!("the persons name is {}", name);


    println!("the persons age from preson struct is {}", person.age);
}