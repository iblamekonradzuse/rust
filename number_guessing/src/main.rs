use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number!");
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read the line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

        println!("Would you like to play again? (yes/no)");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read the line");

        if play_again.trim().to_lowercase() != "yes" {
            break;
        }
    }

    println!("Thanks for playing!");
}
