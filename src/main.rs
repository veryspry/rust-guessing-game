use std::io;
use std::cmp::Ordering;
use rand::Rng;
    
fn main() {
    println!("Guess the number!");

    let random_number: u32 = rand::thread_rng()
        .gen_range(1..101);

    let mut prompt_message = "Please input a guess...";

    println!("The secret number is {}", random_number);

    let mut guess = String::new();

    loop {
        println!("{}", prompt_message);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let parsed_guess: u32 = guess.parse()
            .expect("Please type a number!");

        println!("You guessed {}", guess);

        match parsed_guess.cmp(&random_number) {
            Ordering::Less => {
                prompt_message = "Too small, try again";
            },
            Ordering::Greater => {
                prompt_message = "Too big, try again";
            },
            Ordering::Equal => {
                println!("Just right!");
                break;
            },
        }
    }

    println!("The secret number was {}", random_number);
}
