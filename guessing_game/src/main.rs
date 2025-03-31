use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Please, guess a number between 1 and 100");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut chances_left: u8 = 10;

    loop {
        if chances_left < 1 {
            println!("You have used up all chances. Gave over! You Lose!");
            break;
        }
        println!("Please, input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };

        chances_left -= 1;
        let mut noun = "chances";
        if chances_left > 0 {
            if chances_left == 1 {
                noun = "chances";
            } 
            println!("You have {} {} left!", chances_left, noun);
        }
    }
}