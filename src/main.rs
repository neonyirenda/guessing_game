use std::io::{self, Write};
use std::cmp;
use rand::Rng;

fn main() {
    println!("Guessing Game!\n\n");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter Number: ");

        io::stdout()
            .flush()
            .expect("Unable to flash screen!");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read_line");

        let input: i32 = match input
            .trim()
            .parse() {
                Ok(input) => input,
                Err(e) => {
                    println!("Error: {e}, Enter numbers between 1 and 100");
                    continue
                },
            };

        match input.cmp(&secret) {
            cmp::Ordering::Greater => println!("Lower."),
            cmp::Ordering::Less => println!("Higher."),
            cmp::Ordering::Equal => {
                println!("You Guessed it, the number is {secret}");
                break
            },
        }
    }
}
