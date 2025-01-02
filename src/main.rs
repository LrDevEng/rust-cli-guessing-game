use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 1;

    println!("Guess a number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                if count == 1 {
                    println!("You win in 1 try!");
                } else {
                    println!("You win in {count} tries!");
                }
                break;
            },
        }

        count += 1;
    }
}
