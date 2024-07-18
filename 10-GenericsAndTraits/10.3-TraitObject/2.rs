trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("look the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("look the duck.. sorry the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", { "swan swan" });
    }
}

fn main() {
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    for bird in birds {
        bird.quack();
    }
}
