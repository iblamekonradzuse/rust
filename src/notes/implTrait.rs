trait Iterator {
    fn next(&mut self) -> Option<i32>;
}

struct Counter {
    count: i32,
}

impl Iterator for Counter {
    fn next(&mut self) -> Option<i32> {
        self.count += 1;
        Some(self.count)
    }
}

fn get_iterator() -> impl Iterator {
    Counter { count: 0 }
}

fn main() {
    let mut iter = get_iterator();
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
}
