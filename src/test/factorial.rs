fn factorial(x: u32) -> u32 {
    (1..=x).product()
}

fn main() {
    let x = 6; // Example number
    let result = factorial(x);
    println!("The factorial of {} is {}", x, result);
}
