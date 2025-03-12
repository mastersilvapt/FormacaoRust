use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    let rnd: u32 = rand::rng().random_range(1..50);

    println!("Guess the number between 1 50");
    let mut input = String::new();

    loop {
        print!("Your guess: ");
        stdout().flush().unwrap();
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();
        if input == rnd {
            break;
        }
        if input < rnd {
            println!("The random number is greater than {}", input);
        }else {
            println!("The random number is smaller than {}", input);
        }
    }
    println!("Congratulations you guessed the number correctly {}!", rnd);
}
