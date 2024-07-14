struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "mooooo!".to_string()
    }
}

fn random_animal<'a>(random_number: f64) -> &'a dyn Animal {
    if random_number < 0.5 {
        &Sheep {}
    } else {
        &Cow {}
    }
}
fn main() {
    let random_number = 0.734;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
